// src/commands/donate.rs
use crate::imports::*;
use serde_json::Value;
use std::fs;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;
use serenity::builder::CreateEmbedFooter;

// Helper function to select a random background image based on weight
fn select_random_banner(banners: &Vec<Value>) -> &str {
    let weights: Vec<_> = banners.iter().map(|b| b["weight"].as_u64().unwrap_or(1) as u32).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let index = dist.sample(&mut rng);
    banners[index]["url"].as_str().unwrap()
}

pub async fn handle_donate_command(ctx: &Context, msg: &Message) {
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

    // Create the donation message
    let donate_message = CreateMessage::new().embed(
        CreateEmbed::new()
            .color(color)
            .image(background_image_url)
            .author(CreateEmbedAuthor::new(author_name).icon_url(author_icon_url))
            .footer(CreateEmbedFooter::new("x.com/coinchimpx"))
            .field("Donation Address", "kaspa:qrt3lf6jejjdzwtnvlr3z35w7j6q66gt49a7grdwsq98nmlg5uz97whuf8qfr", false)
            .description("[Check Kaspa Donation Wallet Balance](https://kas.fyi/address/kaspa:qrt3lf6jejjdzwtnvlr3z35w7j6q66gt49a7grdwsq98nmlg5uz97whuf8qfr)")
            .image("https://nachowyborski.xyz/donation_wallet.png"),
    );

    if let Err(why) = msg.channel_id.send_message(ctx.http.clone(), donate_message).await {
        println!("Error sending message: {:?}", why);
    }
}
