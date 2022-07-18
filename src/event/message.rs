use line_bot_sdk::{
    models::{
        message::{text::TextMessage, MessageObject},
        webhook_event::{Event, Message},
    },
    Client,
};

use crate::error::AppError;

pub async fn index(
    _client: &Client,
    event: &Event,
) -> Result<Option<Vec<MessageObject>>, AppError> {
    let message = match &event.message {
        Some(message) => message,
        None => return Err(AppError::BadRequest("Message not found".to_string())),
    };
    let text_message = match message {
        Message::Text(text_message) => text_message,
        _ => return Err(AppError::BadRequest("Message is not text".to_string())),
    };
    Ok(Some(vec![TextMessage::builder()
        .text(&text_message.text)
        .build()
        .into()]))
}
