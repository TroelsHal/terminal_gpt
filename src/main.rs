//! The main module for the Terminal GPT application.
//!
//! This module sets up and runs the main event loop of the application, handling
//! user input, processing it using the OpenAI API, and displaying responses.
//! It demonstrates basic interaction with the OpenAI API using a command-line interface.

mod client;
mod errors;
mod interaction;
mod request_body;

use errors::CustomError;
use interaction::{process_user_input, read_user_input};
use request_body::{OpenAIRequestBody, Role};
use reqwest::Url;
use std::env;
use std::io::{stdout, Stdout};

/// The entry point of the application.
///
/// It initializes necessary resources such as the HTTP client and
/// runs the main loop to process user input and interact with the OpenAI API.
#[tokio::main]
async fn main() {
    // Retrieve OpenAI API key from environment variable.
    let openai_api_key_env_var = "OPENAI_API_KEY_RUSTPROJECT";
    let key = match env::var(openai_api_key_env_var) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Failed to get API key: {}", e);
            return;
        }
    };

    // Build the HTTP client for interacting with the OpenAI API.
    let client_builder = client::ClientBuilder::new(&key);
    let client = match client_builder.build() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to build client: {}", e);
            return;
        }
    };

    // Prepare the request body structure for the OpenAI API.
    let mut request_body = OpenAIRequestBody {
        model: String::from("gpt-3.5-turbo"),
        messages: vec![],
    };

    // Add initial message to the request body.
    request_body.add_new_message(Role::System, "You are a helpful assistant".to_string());

    // Parse the URL for the OpenAI API endpoint.
    let url = match Url::parse("https://api.openai.com/v1/chat/completions") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Invalid URL: {}", e);
            return;
        }
    };

    // Initialize stdout for writing output to the terminal.
    let mut stdout: Stdout = stdout();
    // Buffer for storing user input.
    let mut user_input: String = String::new();

    // Instruction for users on how to exit the application.
    println!("Let's go. (Quit by typing exit.)");

    // Main loop for reading and processing user input.
    while let Some(trimmed_user_input) = read_user_input(&mut user_input, &mut stdout).await {
        // Check for exit command.
        if trimmed_user_input.eq_ignore_ascii_case("exit") {
            break;
        }

        // Process the user input and handle potential errors.
        match process_user_input(&client, &url, &mut request_body, trimmed_user_input).await {
            Ok(_) => {
                // Continue to the next iteration upon successful processing.
            }
            Err(e) => {
                // Handle different errors based on their type.
                match e {
                    CustomError::Serialization(err) => {
                        eprintln!("Serialization error: {}", err);
                    }
                    CustomError::Request(err) => {
                        eprintln!("Request error: {}", err);
                    }
                    CustomError::Response(status) => {
                        eprintln!("Server responded with status: {}", status);
                    }
                    CustomError::ReadResponse(err) => {
                        eprintln!("Error reading response: {}", err);
                    }
                    CustomError::ParseJson(err) => {
                        eprintln!("Error parsing JSON: {}", err);
                    }
                    CustomError::NoMessageFound => {
                        eprintln!("No message found in response.");
                    }
                }
                continue;
            }
        }
    }
}
