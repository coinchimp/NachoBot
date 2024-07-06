pub use dotenv::dotenv; //Loads environment variables from `.env`
pub use serde::{Deserialize, Serialize}; //Used for serializing (Convert for easy storage and use) data
pub use serde_json::json; //Used for creating JSon files
pub use reqwest::Error; //Used for handling errors related to HTTP requests

pub use std::env; //Interact with environment
pub use std::fs::{File, OpenOptions}; // Used for file operations: `File` for creating/reading/writing, `OpenOptions` for configuring file options
pub use std::io::prelude::*; // Brings in traits for I/O operations like Read and Write
pub use std::io::{self, Read, Write}; // I/O operations: `io` module for general I/O, `Read` and `Write` for reading from and writing to files
pub use std::path::Path; // Used for handling and manipulating file system paths
pub use std::time::{SystemTime, UNIX_EPOCH}; // Used for time operations: `SystemTime` for current system time, `UNIX_EPOCH` for UNIX epoch time

pub use serenity::async_trait; // Provides support for async traits
pub use serenity::model::channel::Message; // Message sent in channel
pub use serenity::model::gateway::GatewayIntents;
pub use serenity::model::gateway::Ready; // Event for when bot is ready
pub use serenity::builder::CreateEmbed; // Create embeds
pub use serenity::builder::CreateEmbedAuthor; // Create embedded author
pub use serenity::builder::CreateMessage; // Create messages (can be embed)
pub use serenity::prelude::*; // Commonly used traits and types from serenity