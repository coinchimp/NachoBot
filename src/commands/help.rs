use crate::imports::*;
use serde_json::Value;
use std::fs;
use serenity::builder::CreateEmbedFooter;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;

pub async fn handle_help_command(ctx: &Context, msg: &Message) {
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

    // Load the help content from the JSON file
    let help_content = match fs::read_to_string("help_content.json") {
        Ok(content) => content,
        Err(e) => {
            println!("Failed to read help content: {:?}", e);
            return;
        }
    };

    let help_json: Value = match serde_json::from_str(&help_content) {
        Ok(json) => json,
        Err(e) => {
            println!("Failed to parse help content: {:?}", e);
            return;
        }
    };

    let commands = help_json["commands"].as_array().expect("Commands not found in help content");

    let mut embed = CreateEmbed::new()
        .color(color)
        .image(background_image_url)
        .author(CreateEmbedAuthor::new(author_name).icon_url(author_icon_url))
        .footer(CreateEmbedFooter::new("x.com/coinchimpx"));

    // Add content and footer
    let content = format!("**# Help Menu**");

    // Add the commands to the embed
    for command in commands {
        let name = command["name"].as_str().unwrap_or("Unknown Command");
        let description = command["description"].as_str().unwrap_or("No description available");
        embed = embed.field(name, description, false);
    }

    let payload = CreateMessage::new()
        .content(content)
        .embed(embed);

    if let Err(why) = msg.channel_id.send_message(ctx.http.clone(), payload).await {
        println!("Error sending message: {:?}", why);
    }
}

// Helper function to select a random background image based on weight
fn select_random_banner(banners: &Vec<Value>) -> &str {
    let weights: Vec<_> = banners.iter().map(|b| b["weight"].as_u64().unwrap_or(1) as u32).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let index = dist.sample(&mut rng);
    banners[index]["url"].as_str().unwrap()
}
