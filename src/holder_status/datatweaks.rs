//use crate::DataStruct; // Import the DataStruct from the current crate
use crate::imports::*; // Import everything from the imports module
use serde_json::Value;
use std::fs;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;
use serenity::builder::CreateEmbedFooter;
//use chrono::{Utc, TimeZone};

// Define a struct for handling holder data
#[derive(Debug, Serialize, Deserialize)]
pub struct HolderData {
    pub message: String,
    pub prev: String,
    pub next: String,
    pub result: Vec<TokenInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub tick: String,
    pub balance: String,
    pub locked: String,
    pub dec: String,
    pub opScoreMod: String,
}

// Helper function to select a random background image based on weight
fn select_random_banner(banners: &Vec<Value>) -> &str {
    let weights: Vec<_> = banners.iter().map(|b| b["weight"].as_u64().unwrap_or(1) as u32).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let index = dist.sample(&mut rng);
    banners[index]["url"].as_str().unwrap()
}

// Helper function to format large numbers
fn format_large_number(number: f64) -> String {
    const BILLION: f64 = 1_000_000_000.0;
    const MILLION: f64 = 1_000_000.0;
    const THOUSAND: f64 = 1_000.0;

    if number >= BILLION {
        format!("{:.2}B", number / BILLION)
    } else if number >= MILLION {
        format!("{:.2}M", number / MILLION)
    } else if number >= THOUSAND {
        format!("{:.2}K", number / THOUSAND)
    } else {
        format!("{}", number)
    }
}

// Fetch data from the API for the given wallet address
pub async fn fetch_holder_data(api_base_url: &str, address: &str) -> Result<HolderData, Error> {
    let url = format!("{}/address/{}/tokenlist", api_base_url, address); // Construct the API URL using the base URL
    let response = reqwest::get(url).await?.json::<HolderData>().await?; // Send a GET request and parse the JSON response
    Ok(response)
}

// Format the fetched holder data into a message to be sent
pub async fn format_holder_data(data: HolderData, address: &str) -> CreateMessage {
    // Load the JSON template
    let template_content = fs::read_to_string("message_template.json").expect("Failed to read message template");
    let template: Value = serde_json::from_str(&template_content).expect("Failed to parse message template");

    // Ensure required fields are present in the template
    let color = template["color"].as_u64().expect("Color not found in message template") as u32;
    let background_images = template["background_images"].as_array().expect("Background images not found in message template");
    let background_image_url = select_random_banner(background_images);
    let author_name = template["author"]["name"].as_str().expect("Author name not found in message template");
    let author_icon_url = template["author"]["icon_url"].as_str().expect("Author icon URL not found in message template");

    // Create the message payload with an embedded message
    let mut embed = CreateEmbed::new()
        .color(color)
        .image(background_image_url)
        .author(CreateEmbedAuthor::new(author_name).icon_url(author_icon_url))
        .footer(CreateEmbedFooter::new("x.com/coinchimpx"));

    // Add content and footer
    let content = format!("**# KRC20 Balance**");
    // Format each token holding information
    embed = embed.field("Address", address ,false);

    for token in data.result.iter() {
        let balance = token.balance.parse::<f64>().expect("Not a valid f64") / 10f64.powf(token.dec.parse::<f64>().expect("Not a valid f64"));
        let formatted_balance = format_large_number(balance);
        embed = embed.field(token.tick.to_uppercase(), formatted_balance, true);
    }

    CreateMessage::new()
        .content(content)
        .embed(embed)
}
