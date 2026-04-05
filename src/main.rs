use rustmimsms::{send_bulk_sms, send_sms};

#[tokio::main]
async fn main() {
    // Use the functions here for testing
    // Use real phone numbers and messages when testing

    match send_sms("1XXXXXXXXX", "Your message here.").await {
        Ok(_) => println!("Success! SMS sent."),
        Err(e) => eprintln!("Error: {}", e),
    }

    match send_bulk_sms(
        vec!["1XXXXXXXX", "1XXXXXXXXXX"],
        "Your broadcast message here.",
    )
    .await
    {
        Ok(_) => println!("Success! Bulk SMS sent."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
