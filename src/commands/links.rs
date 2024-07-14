// src/commands/links.rs
use crate::imports::*;
use serde_json::Value;
use std::fs;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;
use serenity::builder::CreateEmbedFooter; // Add this import

// Helper function to select a random background image based on weight
fn select_random_banner(banners: &Vec<Value>) -> &str {
    let weights: Vec<_> = banners.iter().map(|b| b["weight"].as_u64().unwrap_or(1) as u32).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let index = dist.sample(&mut rng);
    banners[index]["url"].as_str().unwrap()
}

pub async fn handle_links_command(ctx: &Context, msg: &Message) {
    // Load the JSON file
    let links_content = match fs::read_to_string("nacho_links.json") {
        Ok(content) => content,
        Err(e) => {
            println!("Failed to read links file: {:?}", e);
            return;
        }
    };

    let links: Value = match serde_json::from_str(&links_content) {
        Ok(links) => links,
        Err(e) => {
            println!("Failed to parse links file: {:?}", e);
            return;
        }
    };

    // Load the JSON template
    let template_content = match fs::read_to_string("message_template.json") {
        Ok(content) => content,
        Err(e) => {
            println!("Failed to read message template: {:?}", e);
            return;
        }
    };

    let template: Value = match serde_json::from_str(&template_content) {
        Ok(template) => template,
        Err(e) => {
            println!("Failed to parse message template: {:?}", e);
            return;
        }
    };

    // Ensure required fields are present in the template
    let color = template["color"].as_u64().expect("Color not found in message template") as u32;
    let background_images = template["background_images"].as_array().expect("Background images not found in message template");
    let background_image_url = select_random_banner(background_images);
    let author_name = template["author"]["name"].as_str().expect("Author name not found in message template");
    let author_icon_url = template["author"]["icon_url"].as_str().expect("Author icon URL not found in message template");

    // Ensure required fields are present in the links
    let links_array = links["links"].as_array().expect("Links array not found in links file");

    // Create the content for the message
    let content = format!("**# Official Links**");

    // Create the message payload with an embedded message
    let mut embed = CreateEmbed::new()
        .color(color)
        .image(background_image_url)
        .author(CreateEmbedAuthor::new(author_name).icon_url(author_icon_url))
        .footer(CreateEmbedFooter::new("x.com/coinchimpx"));

    for link in links_array {
        let name = link["name"].as_str().expect("Link name not found");
        let url = link["url"].as_str().expect("Link URL not found");
        embed = embed.field(name, url, false);
    }

    let payload = CreateMessage::new()
        .content(content) // Add this line
        .embed(embed);

    if let Err(why) = msg.channel_id.send_message(ctx.http.clone(), payload).await {
        println!("Error sending message: {:?}", why);
    }
}
