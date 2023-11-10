/// A module for handling user interaction in the terminal chat application.
///
/// This module contains functions to read user input from the terminal and
/// process it by making requests to the OpenAI API.
use crate::errors::CustomError;
use crate::request_body::{OpenAIRequestBody, Role};
use reqwest::{Client, Url};
use serde_json::Value;
use std::io::{self, Stdout, Write};

/// Reads user input from the terminal.
///
/// This function prompts the user for input, reads the input line from the standard input,
/// and returns it as a `String`. It returns `None` if there's an error in reading input or flushing stdout.
///
/// # Arguments
///
/// * `user_input` - A mutable reference to a `String` where the input is stored.
/// * `stdout` - A mutable reference to `Stdout` for handling the output stream.
pub async fn read_user_input(user_input: &mut String, stdout: &mut Stdout) -> Option<String> {
    print!("\n-->You: ");
    if stdout.flush().is_err() {
        eprintln!("Error flushing stdout");
        return None;
    }

    user_input.clear(); // Clear the input buffer

    if io::stdin().read_line(user_input).is_err() {
        eprintln!("Error reading user input");
        return None;
    }

    Some(user_input.trim().to_string())
}

/// Processes the user input by sending it to the OpenAI API and handling the response.
///
/// This function takes the user input, adds it to the request body, and makes a POST request
/// to the OpenAI API. It then handles the response, extracting and printing the message from GPT.
/// It returns a `Result` indicating the success or failure of the process.
///
/// # Arguments
///
/// * `client` - A reference to the `Client` used for making HTTP requests.
/// * `url` - A reference to the `Url` of the OpenAI API.
/// * `request_body` - A mutable reference to `OpenAIRequestBody` which stores the request data.
/// * `user_input` - The user input as a `String`.
pub async fn process_user_input(
    client: &Client,
    url: &Url,
    request_body: &mut OpenAIRequestBody,
    user_input: String,
) -> Result<(), CustomError> {
    request_body.add_new_message(Role::User, user_input);

    let request_body_string =
        serde_json::to_string(request_body).map_err(CustomError::Serialization)?;

    let response = client
        .post(url.clone())
        .body(request_body_string)
        .send()
        .await
        .map_err(CustomError::Request)?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(CustomError::Response(response.status()));
    }

    let response_body = response.text().await.map_err(CustomError::ReadResponse)?;

    let response_json_value: Value =
        serde_json::from_str(&response_body).map_err(CustomError::ParseJson)?;

    let assistant_message = response_json_value["choices"][0]["message"]["content"]
        .as_str()
        .ok_or(CustomError::NoMessageFound)?
        .to_owned();

    println!("\n-->GPT: {}", assistant_message);
    request_body.add_new_message(Role::Assistant, assistant_message);

    Ok(())
}
