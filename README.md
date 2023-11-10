# Terminal GPT

Terminal GPT is a Rust-based command-line interface application that interacts with the OpenAI API. It allows users to send messages and receive responses from an AI model directly in the terminal.

## Features

- **User Interaction**: Simple command-line interface to input user messages.
- **OpenAI Integration**: Sends user messages to the OpenAI API and displays AI-generated responses.
- **Error Handling**: Custom error handling to manage different types of operational failures.

## Getting Started

### Prerequisites

- Ensure you have Rust and Cargo installed. If not, install them from [rust-lang.org](https://www.rust-lang.org/tools/install).

- OpenAI API key.

### Configuration

Set up your OpenAI API key as an environment variable:
```bash
export OPENAI_API_KEY_RUSTPROJECT="your_api_key_here"
```
Replace "your_api_key_here" with your actual OpenAI API key.

### Building and Running the Application

Navigate to the project directory and run:

```bash
cargo test
```

## Usage

After starting the application, you can type messages and see responses from the OpenAI API. Type exit to quit the application.

## Modules

    client: Handles HTTP client configuration and creation.
    errors: Defines custom error types for the application.
    interaction: Manages user input reading and processing with OpenAI API.
    request_body: Structures the request body for API interaction.

## Author

Code written by Troels Halgreen

## Contributing

Contributions to improve Terminal GPT are welcome. Feel free to fork the project and submit pull requests.
