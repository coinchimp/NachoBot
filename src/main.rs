#![allow(non_snake_case)] // Allow non-snake-case style for identifiers
#[warn(non_camel_case_types)] // Warn if non-camel-case types are used

mod imports; // Import the imports module
pub use imports::*; // Use all items from the imports module
mod datatweaks; // Import the datatweaks module

// Struct to hold the result data, with serialization and deserialization capabilities
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultStruct {
    tick: String,
    max: String,
    lim: String,
    pre: String,
    to: String,
    dec: String,
    minted: String,
    opScoreAdd: String,
    opScoreMod: String,
    state: String,
    hashRev: String,
    mtsAdd: String,
}

// Struct to hold the overall data, which includes a message and a list of results
#[derive(Debug, Serialize, Deserialize)]
pub struct DataStruct {
    message: String,
    result: Vec<ResultStruct>
}

// Struct to handle events
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Handle incoming messages
    async fn message(&self, _ctx: Context, msg: Message) {
        // Ignore messages from bots
        if msg.author.bot {
            return;
        }
        // If the message content is "!track"
        if msg.content == "!track" {
            // Check if 5 minutes have passed since the last fetch
            let fetch_result = if datatweaks::check_time().expect("Failed to fetch data from metadata.json") {
                // If time has passed, fetch data from the API
                match datatweaks::fetch_from_api().await {
                    Ok(data) => {
                        // Save the fetched data
                        datatweaks::save_data(&data).expect("Failed to save data");
                        data
                    },
                    Err(e) => {
                        // Handle fetch error
                        println!("Failed to fetch from API: {}", e);
                        return;
                    }
                }
            } else {
                // If time hasn't passed, fetch data from the local JSON file
                match datatweaks::fetch_from_json("data.json") {
                    Ok(data) => data,
                    Err(e) => {
                        // Handle fetch error
                        println!("Failed to fetch from JSON: {}", e);
                        return;
                    }
                }
            };

            // Format the fetched data into a message
            let formatted_message = datatweaks::format_data(fetch_result).await;
            // Send the message to the channel
            if let Err(why) = msg.channel_id.send_message(&_ctx.http, formatted_message).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Handle the ready event
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name); // Print a message when the bot is connected
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from a `.env` file
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment"); // Get the Discord token from the environment variables

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT; // Set the intents for the bot

    // Create a new client instance
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler) // Set the event handler
        .await
        .expect("Err creating client");

    // Start the client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why); // Handle client start error
    }
}
