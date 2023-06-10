use serde::{Deserialize, Serialize};
use reqwest::Client;

use reqwest::Error;
use crate::utils;

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    data: Data,
}


#[derive(Debug, Deserialize, Serialize)]
struct Data {
    // Add fields that you need from the data object
    // Based on the example response, you can include fields for cryptocurrencies with IDs 1 and 2
    // Modify the field types accordingly based on the actual data types in the response
    #[serde(rename = "1")]
    crypto_1: Cryptocurrency,

    #[serde(rename = "2")]
    crypto_2: Cryptocurrency,

    #[serde(rename = "3")]
    crypto_3: Cryptocurrency,

    #[serde(rename = "4")]
    crypto_4: Cryptocurrency,
}

#[derive(Debug, Deserialize, Serialize)]
struct Cryptocurrency {
    id: u32,
    name: String,
    symbol: String,
    // Add other fields as needed
    quote: Quote,
}

#[derive(Debug, Deserialize, Serialize)]
struct Quote {
    #[serde(rename = "USD")]

    usd: QuoteDetails,
}

#[derive(Debug, Deserialize, Serialize)]
struct QuoteDetails {
    price: f64,
    volume_24h: f64,
    // Add other fields as needed
}


pub async fn crypto() -> Result<(), Error> {
    let client = Client::new();

    let url = "https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest";

    let params = [
        ("id", "1,2,3,4"),
        ("convert", "USD"),    // Convert market values to USD
    ];

    let response = client.get(url).header("X-CMC_PRO_API_KEY", utils::load_api_key()).query(&params).send().await?;


    // Handle the response as per your requirements
    let response_text = response.text().await?;

    let result: ApiResponse = serde_json::from_str(&*response_text).unwrap();

    println!("{:#?}", result);
    Ok(())
}
