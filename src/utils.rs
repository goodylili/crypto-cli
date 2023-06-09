use std::env;
use dotenv::dotenv;


pub fn load_api_key() -> String {
    dotenv().ok();

    match env::var("CMC_KEY") {
        Ok(api_key) => api_key,
        Err(_) => {
            eprintln!("CMC_KEY not found in .env file");
            std::process::exit(1);
        }
    }
}