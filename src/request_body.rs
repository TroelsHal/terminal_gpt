use serde::Serialize;

/// Represents the role of a message in the OpenAI request.
///
/// This enum is used to specify whether a message is from the system, the user, or the assistant.
pub enum Role {
    System,
    User,
    Assistant,
}

impl Role {
    /// Returns a static string slice representing the role.
    ///
    /// This method is typically used to convert the enum variant into a string
    /// that can be serialized or displayed.
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        }
    }
}

/// Represents a request body for the OpenAI API.
///
/// This struct is used to serialize data into a format suitable for
/// interacting with the OpenAI API. It contains the model name and a list of messages.
#[derive(Serialize, Debug)]
pub struct OpenAIRequestBody {
    /// The model identifier used for the OpenAI request.
    pub model: String,
    /// A collection of messages that form part of the request.
    pub messages: Vec<Message>,
}

/// Represents a message within an OpenAI API request.
///
/// Each message consists of a role (system, user, or assistant) and the content of the message.
#[derive(Serialize, Debug)]
pub struct Message {
    /// The role associated with the message (system, user, or assistant).
    pub role: &'static str,
    /// The textual content of the message.
    pub content: String,
}

impl OpenAIRequestBody {
    /// Adds a new message to the request body.
    ///
    /// This method takes a `Role` and a `String` as parameters and adds them as a `Message` to the `messages` vector.
    ///
    /// # Arguments
    ///
    /// * `role` - A `Role` enum indicating the role of the message.
    /// * `content` - A `String` containing the message content.
    pub fn add_new_message(&mut self, role: Role, content: String) {
        self.messages.push(Message {
            role: role.as_str(),
            content,
        });
    }
}
