mod api;
mod cli;
mod utils;


use reqwest::Client;

async fn get_crypto() {
    // Create a new Reqwest client
    let client = Client::new();

    // Send a GET request to the API endpoint
    let response = client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/map")
        .header("X-CMC_PRO_API_KEY", utils::load_api_key())
        .send();

    // Check if the request was successful
    match response.await {
        Ok(response) => {
            // Check the status code
            match response.status() {
                reqwest::StatusCode::OK => {
                    // Request was successful, print the response body
                    let body = response.text().await.unwrap();
                    println!("{}", body);
                }
                _ => {
                    // Request failed, print the status code and error message
                    println!("Request failed with status code: {}", response.status());
                }
            }
        }
        Err(error) => {
            // Request failed, print the error message
            println!("Request failed with error: {:?}", error);
        }
    }
}


#[tokio::main]
async fn main() {
    get_crypto().await;

}