#![allow(non_snake_case)]
#[warn(non_camel_case_types)]

// Import necessary modules and make them publicly available
mod imports;
pub use imports::*;
mod mint_status {
    pub mod datatweaks;
}    
mod result_struct;
mod commands {
    pub mod status;
}

// Import the ResultStruct to use in main.rs
use crate::result_struct::ResultStruct;

// Define a struct for handling API data
#[derive(Debug, Serialize, Deserialize)]
pub struct DataStruct {
    message: String,
    result: Vec<ResultStruct>
}

// Define a struct for handling events
struct Handler {
    api_base_url: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore messages from bots
        if msg.author.bot {
            return;
        }

        // Split the message into parts and extract the command
        let message = msg.content.as_str();
        let mut message_parts = message.split_whitespace();
        let command = message_parts.next().unwrap();

        // Call the status command handler
        if command == "!mint_status" {
            commands::status::handle_status_command(&ctx, &msg, &mut message_parts, &self.api_base_url).await;
        }
    }

    // Handle the "ready" event when the bot is connected
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Read the API base URL from the environment variable or default to testnet
    let api_base_url = env::var("KASPLEX_API_BASE_URL").unwrap_or_else(|_| "https://tn11api.kasplex.org/v1/krc20".to_string());

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let discord_task = tokio::spawn(async move {
        let mut client = Client::builder(&token, intents)
            .event_handler(Handler { api_base_url }) // Pass the api_base_url to the handler
            .await
            .expect("Err creating client");

        // Start the client
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    });

    // Get the port from the environment variables, default to 8080 if not set
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap();

    // Set up the health check route
    let health_route = warp::get()
        .and(warp::path::end())
        .map(|| "Healthy");

    // Serve the health check route
    let warp_task = warp::serve(health_route)
        .run(([0, 0, 0, 0], port));

    // Run both tasks concurrently
    tokio::select! {
        _ = discord_task => {},
        _ = warp_task => {},
    }
}
