use line_bot_sdk::Client;
use log::info;

use actix_web::{HttpResponse, Responder};
use line_bot_sdk::extractor::CustomHeader;
use line_bot_sdk::models::message::MessageObject;
use line_bot_sdk::models::webhook_event;

use serde::{Deserialize, Serialize};

use crate::config;
use crate::error::AppError;
use crate::event::message;

pub async fn handler(
    context: String,
    custom_header: CustomHeader,
) -> Result<impl Responder, AppError> {
    info!("Request body: {}", context);

    let client = Client::new(
        config::get_token().map_err(AppError::Env)?,
        config::get_secret().map_err(AppError::Env)?,
        config::get_id().map_err(AppError::Env)?,
    );

    let signature = &custom_header.x_line_signature;
    client
        .verify_signature(signature, &context)
        .map_err(AppError::LineBotSdkError)?;

    let context: webhook_event::Root =
        serde_json::from_str(&context).map_err(AppError::SerdeJson)?;
    webhook_handler(context, client).await
}

async fn webhook_handler(
    context: webhook_event::Root,
    client: Client,
) -> Result<HttpResponse, AppError> {
    for event in &context.events {
        let reply_messages = match event.type_field.as_str() {
            "message" => message::index(&client, event).await,
            // "follow" => follow::index().await,
            _ => return Err(AppError::BadRequest("Unknown event type".to_string())),
        }?;
        if let Some(reply_messages) = reply_messages {
            let reply_token = event
                .reply_token
                .as_ref()
                .ok_or_else(|| AppError::BadRequest("Reply token not found".to_string()))?;
            client
                .reply(reply_token, reply_messages, None)
                .await
                .map_err(AppError::LineBotSdkError)?;
        }
    }
    Ok(HttpResponse::Ok().json("Ok"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReplyMessage {
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    messages: Vec<MessageObject>,
}
