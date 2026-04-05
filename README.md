# RustSMS

A Rust library for sending SMS messages using the MIM SMS API provided by Robi.

## Description

RustMIMSMS is a simple, asynchronous Rust crate that provides an easy-to-use interface for sending autonomous SMS messages through the MIM SMS service. It supports both single SMS sending and bulk SMS broadcasting to multiple recipients.

## Features

- Send single SMS messages
- Send bulk SMS to multiple recipients
- Asynchronous operations using Tokio
- Environment variable configuration
- Error handling with detailed responses

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustmimsms = "0.1.0"
```

## Usage

### Environment Setup

Before using the library, set up the following environment variables:

- `MIM_SMS_API_KEY`: Your MIM SMS API key
- `MIM_SMS_USER`: Your MIM SMS username
- `MIM_SMS_SENDER_ID`: Your sender ID

You can create a `.env` file in your project root:

```
MIM_SMS_API_KEY=your_api_key_here
MIM_SMS_USER=your_username_here
MIM_SMS_SENDER_ID=your_sender_id_here
```

### Sending a Single SMS

```rust
use rustmimsms::send_sms;

#[tokio::main]
async fn main() {
    match send_sms("1XXXXXXXXX", "Hello from RustSMS!").await {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Sending Bulk SMS

```rust
use rustmimsms::send_bulk_sms;

#[tokio::main]
async fn main() {
    let numbers = vec!["1XXXXXXXXX", "1YYYYYYYYY"];
    match send_bulk_sms(numbers, "Bulk message from RustSMS!").await {
        Ok(_) => println!("Bulk SMS sent successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## API Reference

### `send_sms(phone: &str, message: &str) -> Result<String, Box<dyn std::error::Error>>`

Sends a single SMS message to the specified phone number.

- `phone`: The recipient's phone number (without country code, will be prefixed with "880" for Bangladesh)
- `message`: The SMS message content
- Returns: `Ok(String)` with success message or `Err` with error details

### `send_bulk_sms(numbers: Vec<&str>, message: &str) -> Result<(), Box<dyn std::error::Error>>`

Sends the same SMS message to multiple recipients.

- `numbers`: A vector of phone numbers (without country code)
- `message`: The SMS message content
- Returns: `Ok(())` on success or `Err` with error details

## Dependencies

- `dotenvy`: For loading environment variables from `.env` file
- `reqwest`: For making HTTP requests (with JSON feature)
- `serde`: For JSON serialization/deserialization
- `tokio`: For asynchronous runtime

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This library is not officially affiliated with MIM SMS. Please ensure you comply with their terms of service and API usage policies.