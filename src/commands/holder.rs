// src/commands/holder.rs
use crate::imports::*;
use crate::holder_status::datatweaks;
use serde_json::Value;
use std::fs;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;

// Helper function to select a random background image based on weight
fn select_random_banner(banners: &Vec<Value>) -> &str {
    let weights: Vec<_> = banners.iter().map(|b| b["weight"].as_u64().unwrap_or(1) as u32).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let index = dist.sample(&mut rng);
    banners[index]["url"].as_str().unwrap()
}

pub async fn handle_holder_command(ctx: &Context, msg: &Message, message_parts: &mut std::str::SplitWhitespace<'_>, api_base_url: &str) {
    let message_word_count = msg.content.split_whitespace().count();

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
    let color = match template["color"].as_u64() {
        Some(color) => color as u32,
        None => {
            println!("Color not found in message template");
            return;
        }
    };

    let background_images = match template["background_images"].as_array() {
        Some(images) => images,
        None => {
            println!("Background images not found in message template");
            return;
        }
    };

    let background_image_url = select_random_banner(background_images);

    let author_name = match template["author"]["name"].as_str() {
        Some(name) => name,
        None => {
            println!("Author name not found in message template");
            return;
        }
    };

    let author_icon_url = match template["author"]["icon_url"].as_str() {
        Some(url) => url,
        None => {
            println!("Author icon URL not found in message template");
            return;
        }
    };

    if message_word_count == 2 {
        let address = message_parts.next().unwrap_or("");
        
        match datatweaks::fetch_holder_data(api_base_url, address).await {
            Ok(data) => {
                let formatted_message = datatweaks::format_holder_data(data, address).await;
                if let Err(why) = msg.channel_id.send_message(ctx.http.clone(), formatted_message).await {
                    println!("Error sending message: {:?}", why);
                }
            },
            Err(e) => {
                let error_message = CreateMessage::new().embed(CreateEmbed::new()
                    .color(color)
                    .image(background_image_url)
                    .field("Error", "Failed to fetch holder data. Please check the address and try again.", false)
                    .author(CreateEmbedAuthor::new(author_name).icon_url(author_icon_url)));
                if let Err(why) = msg.channel_id.send_message(ctx.http.clone(), error_message).await {
                    println!("Error sending message: {:?}", why);
                }
                println!("Failed to fetch holder data: {}", e);
            }
        }
    } else {
        // Use the template to create the parameter error message
        let parameter_error = CreateMessage::new().embed(CreateEmbed::new()
            .color(color)
            .image(background_image_url)
            .field("Wrong Number of Parameters", "Make sure to use the correct format for: `!tokenbalance [wallet-address]`!", false)
            .author(CreateEmbedAuthor::new(author_name).icon_url(author_icon_url)));
        if let Err(why) = msg.channel_id.send_message(ctx.http.clone(), parameter_error).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
