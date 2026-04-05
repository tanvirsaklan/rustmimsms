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

pub fn formatted_phone_number(p: &str) -> String {
    let mut number = p.trim().replace([' ', '-'], ""); // Trim and remove common separators

    if number.starts_with('+') {
        number = number.trim_start_matches('+').to_string(); // Remove leading '+'
    }

    if number.starts_with("880") {
        number = number.trim_start_matches("880").to_string(); // Remove leading "880"
    }

    if number.starts_with('0') {
        number = number.trim_start_matches('0').to_string(); // Remove leading '0'
    }

    if number.is_empty() {
        return String::new(); // Return empty string if no valid number remains
    }

    format!("880{}", number)
}

fn get_env() -> Result<(String, String, String), Box<dyn std::error::Error>> {
    Ok((
        env::var("MIM_SMS_API_KEY")?,
        env::var("MIM_SMS_USER")?,
        env::var("MIM_SMS_SENDER_ID")?,
    ))
}

pub async fn send_sms(phone: &str, message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (api_key, user_name, sender_name) = get_env()?;

    let phone_number = formatted_phone_number(phone);

    let message_text = message.into();
    let client: Client = Client::new();
    let url: &str = "https://api.mimsms.com/api/SmsSending/SMS";

    let payload = SmsRequest {
        api_key,
        mobile_number: phone_number,
        sender_name,
        campaign_name: String::new(),
        user_name,
        transaction_type: "T".to_string(),
        message_id: "".to_string(),
        message: message_text,
        campaign_id: String::new(),
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
        Ok(api_res.result)
    } else {
        Err(format!("SMS Failed: {} - {}", api_res.status, api_res.result).into())
    }
}

/// Sends a single message to multiple phone numbers
pub async fn send_bulk_sms(
    numbers: Vec<&str>,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = Client::new();
    let url: &str = "https://api.mimsms.com/api/SmsSending/OneToMany";

    let (api_key, user_name, sender_name) = get_env()?;

    // 1. Format numbers: Add '88' prefix and join with commas
    let formatted_numbers = numbers
        .iter()
        .map(|num| formatted_phone_number(num))
        .collect::<Vec<String>>()
        .join(",");

    // 2. Build the payload
    let payload = OneToManyRequest {
        user_name,
        api_key,
        mobile_number: formatted_numbers,
        campaign_id: String::new(),
        sender_name,
        transaction_type: "T".to_string(),
        message: message.into(),
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
        Ok(api_res.result)
    } else {
        Err(format!("Bulk SMS Failed: {}", api_res.result).into())
    }
}
