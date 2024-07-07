// Serde for serialization and deserialization
pub use serde::{Deserialize, Serialize}; // Used for serializing (converting for easy storage and use) data
pub use serde_json::json; // Used for creating JSON files

// Reqwest for HTTP requests and error handling
pub use reqwest::Error; // Used for handling errors related to HTTP requests

// Standard library imports for environment interaction and file operations
pub use std::env; // Interact with environment variables
pub use std::fs::{create_dir_all, File, OpenOptions}; // File operations: create directory, file creation/reading/writing, configuring file options
pub use std::io::{self, prelude::*, Read, Write}; // I/O operations: traits for I/O operations like Read and Write, general I/O operations
pub use std::path::Path; // Handling and manipulating file system paths
pub use std::time::{SystemTime, UNIX_EPOCH}; // Time operations: current system time, UNIX epoch time

// Serenity for Discord bot functionality
pub use serenity::async_trait; // Provides support for async traits
pub use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage}; // Create embeds, embedded authors, and messages (can be embeds)
pub use serenity::model::channel::Message; // Message sent in channel
pub use serenity::model::gateway::{GatewayIntents, Ready}; // Event for when the bot is ready, gateway intents
pub use serenity::prelude::*; // Commonly used traits and types from Serenity

pub use warp::Filter;
pub use tokio;