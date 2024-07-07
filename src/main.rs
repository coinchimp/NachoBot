#![allow(non_snake_case)]
#[warn(non_camel_case_types)]

// Import necessary modules and make them publicly available
mod imports;
pub use imports::*;
mod datatweaks;


// Define a struct for handling API results
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

// Define a struct for handling API data
#[derive(Debug, Serialize, Deserialize)]
pub struct DataStruct {
    message: String,
    result: Vec<ResultStruct>
}

// Define a struct for handling events
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        // Ignore messages from bots
        if msg.author.bot {
            return;
        }

        // Split the message into parts and extract the command
        let message = msg.content.as_str();
        let mut message_parts = message.split(' ');
        let command = message_parts.next().unwrap();
        let message_word_count = message.split_whitespace().count();

        // Handle the "!track" command with exactly two parameters
        if command == "!status" && message_word_count == 2 {
            let token = message_parts.next().unwrap_or("").to_uppercase();
            let should_fetch;

            // Check if data needs to be fetched from the API
            let fetch_result = if let Ok((fetch, _)) = datatweaks::check_time(&token) {
                should_fetch = fetch;
                if should_fetch {
                    match datatweaks::fetch_from_api(&token).await {
                        Ok(data) => {
                            // Save the fetched data
                            datatweaks::save_data(&data, &token).expect("Failed to save data");
                            data
                        },
                        Err(e) => {
                            // Handle errors during API fetch
                            if e.to_string().contains("invalid type: null, expected a sequence") {
                                let token_error = CreateMessage::new().embed(CreateEmbed::new()
                                .color(7391162)
                                .field("Invalid token", "Make sure to provide a valid token for: `!status [token]`!", false)
                                .author(
                                    CreateEmbedAuthor::new("Temp")
                                        .name("Nacho the 𐤊at")
                                ));
                                if let Err(why) = msg.channel_id.send_message(&_ctx.http, token_error).await {
                                    println!("Error sending message: {:?}", why);
                                }
                            } else {
                                println!("Failed to fetch from API: {}", e);
                            }
                            return;
                        }
                    }
                } else {
                    // Fetch data from JSON if not fetching from API
                    match datatweaks::fetch_from_json(&token) {
                        Ok(data) => data,
                        Err(e) => {
                            println!("Failed to fetch from JSON: {}", e);
                            return;
                        }
                    }
                }
            } else {
                println!("Failed to check time");
                return;
            };

            // Format the fetched data and send it as a message
            let formatted_message = datatweaks::format_data(fetch_result).await;
            if let Err(why) = msg.channel_id.send_message(&_ctx.http, formatted_message).await {
                println!("Error sending message: {:?}", why);
            } else {
                println!("Token: {}, Should Fetch: {}", token, should_fetch);
            }
        } else if command == "!status" {
            // Handle incorrect number of parameters for the "!track" command
            let paramater_error = CreateMessage::new().embed(CreateEmbed::new()
            .color(7391162)
            .field("Wrong Number of Parameters", "Make sure to use the correct format for: `!status [token]`!", false)
            .author(
                CreateEmbedAuthor::new("Temp")
                    .name("Nacho the 𐤊at")
            ));
            if let Err(why) = msg.channel_id.send_message(&_ctx.http, paramater_error).await {
                println!("Error sending message: {:?}", why);
            }
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

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let discord_task = tokio::spawn(async move {
        let mut client = Client::builder(&token, intents)
            .event_handler(Handler) // Set the event handler
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
