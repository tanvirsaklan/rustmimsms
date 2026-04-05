use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")] // To ensure field names match API expectations
struct SmsRequest {
    api_key: String,
    mobile_number: String,
    sender_name: String,
    campaign_name: String,
    user_name: String,
    transaction_type: String,
    message_id: String,
    message: String,
    campaign_id: String,
    // Use Option to handle nulls gracefully
    sms_data: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")] // To ensure field names match API expectations
struct OneToManyRequest {
    user_name: String,
    api_key: String,
    mobile_number: String, // Comma-separated string
    campaign_id: String,
    sender_name: String,
    transaction_type: String,
    message: String,
}

#[derive(Deserialize, Debug)]
struct SmsResponse {
    #[serde(rename = "statusCode")]
    status_code: String,
    status: String,
    #[serde(rename = "responseResult")]
    result: String,
}

pub async fn send_sms(phone: &str, message: &str) -> Result<String, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    let api_key = env::var("MIM_SMS_API_KEY").unwrap_or_else(|_| "Set env variables correctly".to_string());
    let user_name = env::var("MIM_SMS_USER").unwrap_or_else(|_| "Set env variables correctly".to_string());
    let sender_name = env::var("MIM_SMS_SENDER_ID").unwrap_or_else(|_| "Set env variables correctly".to_string());

    let message_text = message.to_string();
    let client: Client = Client::new();
    let url: &str = "https://api.mimsms.com/api/SmsSending/SMS";

    let payload = SmsRequest {
        api_key,
        mobile_number: format!("880{}", phone),
        sender_name,
        campaign_name: "".to_string(),
        user_name,
        transaction_type: "T".to_string(),
        message_id: "".to_string(),
        message: message_text,
        campaign_id: "null".to_string(),
        sms_data: None,
    };

    // The .json() method will now be found because the "json" feature is in Cargo.toml
    let response = client
        .post(url)
        .header("Authorization", "bearer") 
        .json(&payload) 
        .send()
        .await?;

    let api_res: SmsResponse = response.json().await?;

    if api_res.status_code == "200" {
        Ok("SMS sent successfully.".into())
    } else {
        Err(format!("SMS Failed: {} - {}", api_res.status, api_res.result).into())
    }
}

/// Sends a single message to multiple phone numbers
pub async fn send_bulk_sms(numbers: Vec<&str>, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client: Client = Client::new();
    let url: &str = "https://api.mimsms.com/api/SmsSending/OneToMany";

    let api_key = env::var("MIM_SMS_API_KEY").unwrap_or_else(|_| "Set env variables correctly".to_string());
    let user_name = env::var("MIM_SMS_USER").unwrap_or_else(|_| "Set env variables correctly".to_string());
    let sender_name = env::var("MIM_SMS_SENDER_ID").unwrap_or_else(|_| "Set env variables correctly".to_string());

    // 1. Format numbers: Add '88' prefix and join with commas
    let formatted_numbers = numbers
        .iter()
        .map(|num| format!("880{}", num))
        .collect::<Vec<String>>()
        .join(",");

    // 2. Build the payload
    let payload = OneToManyRequest {
        user_name,
        api_key,
        mobile_number: formatted_numbers,
        campaign_id: "null".to_string(),
        sender_name,
        transaction_type: "T".to_string(),
        message: message.to_string(),
    };

    // 3. Execute request
    let response = client
        .post(url)
        .header("Authorization", "bearer")
        .json(&payload)
        .send()
        .await?;

    let api_res: SmsResponse = response.json().await?;

    if api_res.status_code == "200" {
        println!("Bulk SMS Success: {}", api_res.result);
        Ok(())
    } else {
        Err(format!("Bulk SMS Failed: {}", api_res.result).into())
    }
}

#[tokio::main]
async fn main() {
    // Use the functions here for testing
    // Use real phone numbers and messages when testing

    match send_sms("1XXXXXXXXX", "Your message here.").await {
        Ok(_) => println!("Success! SMS sent."),
        Err(e) => eprintln!("Error: {}", e),
    }

    match send_bulk_sms(vec!["1XXXXXXXX","1XXXXXXXXXX"], "Your broadcast message here.").await {
        Ok(_) => println!("Success! Bulk SMS sent."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
