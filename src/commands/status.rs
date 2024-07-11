// src/commands/status.rs
use crate::imports::*;
use crate::mint_status::datatweaks;
use serde_json::Value;
use std::fs;

pub async fn handle_status_command(ctx: &Context, msg: &Message, message_parts: &mut std::str::SplitWhitespace<'_>, api_base_url: &str) {
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

    let background_image_url = match template["background_image_url"].as_str() {
        Some(url) => url,
        None => {
            println!("Background image URL not found in message template");
            return;
        }
    };

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
        let token = message_parts.next().unwrap_or("").to_uppercase();
        let should_fetch;

        let fetch_result = if let Ok((fetch, _)) = datatweaks::check_time(&token) {
            should_fetch = fetch;
            if should_fetch {
                match datatweaks::fetch_from_api(api_base_url,&token).await {
                    Ok(data) => {
                        datatweaks::save_data(&data, &token).expect("Failed to save data");
                        data
                    },
                    Err(e) => {
                        if e.to_string().contains("invalid type: null, expected a sequence") {
                            // Use the template to create the error message
                            let token_error = CreateMessage::new().embed(CreateEmbed::new()
                                .color(color)
                                .image(background_image_url)
                                .field("Invalid token", "Make sure to provide a valid token for: `!status [token]`!", false)
                                .author(
                                    CreateEmbedAuthor::new(author_name)
                                        .icon_url(author_icon_url)
                                ));
                            if let Err(why) = msg.channel_id.send_message(ctx.http.clone(), token_error).await {
                                println!("Error sending message: {:?}", why);
                            }
                        } else {
                            println!("Failed to fetch from API: {}", e);
                        }
                        return;
                    }
                }
            } else {
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

        let formatted_message = datatweaks::format_data(fetch_result).await;
        if let Err(why) = msg.channel_id.send_message(ctx.http.clone(), formatted_message).await {
            println!("Error sending message: {:?}", why);
        } else {
            println!("Token: {}, Should Fetch: {}", token, should_fetch);
        }
    } else {
        // Use the template to create the parameter error message
        let paramater_error = CreateMessage::new().embed(CreateEmbed::new()
            .color(color)
            .image(background_image_url)
            .field("Wrong Number of Parameters", "Make sure to use the correct format for: `!status [token]`!", false)
            .author(
                CreateEmbedAuthor::new(author_name)
                    .icon_url(author_icon_url)
            ));
        if let Err(why) = msg.channel_id.send_message(ctx.http.clone(), paramater_error).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
