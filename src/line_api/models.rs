use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Food {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "imageUrl")]
    image_url: String,
    #[serde(rename = "maker")]
    maker: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "stock")]
    stock: i64,
}
