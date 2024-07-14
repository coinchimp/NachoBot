use crate::DataStruct; // Import the DataStruct from the current crate
use crate::imports::*; // Import everything from the imports module
use serde_json::Value;
use std::fs;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;
use serenity::builder::CreateEmbedFooter;
use chrono::{ Utc, TimeZone};


const STORAGE_FOLDER: &str = "data_storage"; // Define a constant for the storage folder name
const PERIOD_LIMIT: u64 = 600; // Define time retention in files for cache

// Define a struct for metadata to store the timestamp
#[derive(Serialize, Deserialize)]
struct Metadata {
    timestamp: u64,
}

// Get the current time in seconds since the UNIX epoch
pub fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards") // Handle potential error where system time is before the UNIX epoch
        .as_secs()
}

// Ensure the storage folder exists, creating it if necessary
fn ensure_storage_folder_exists() -> io::Result<()> {
    create_dir_all(STORAGE_FOLDER)
}

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


// Check if five minutes have passed since the last fetch for the given token
pub fn check_time(token: &str) -> io::Result<(bool, bool)> {
    let current_time = current_time(); // Get the current time
    ensure_storage_folder_exists()?; // Ensure the storage folder exists
    let json_name_time = format!("{}/{}_metadata.json", STORAGE_FOLDER, token); // Construct the metadata file path

    // Try to open the metadata file
    let mut file = match OpenOptions::new().read(true).open(&json_name_time) {
        Ok(file) => file,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            return Ok((true, true)); // Return true if it's the first time checking
        }
        Err(e) => return Err(e), // Return the error if there was an issue opening the file
    };

    let mut json = String::new();
    file.read_to_string(&mut json)?; // Read the file contents into a string
    
    let metadata: Metadata = serde_json::from_str(&json)?; // Deserialize the JSON string into Metadata

    let time_period_passed = current_time > metadata.timestamp + PERIOD_LIMIT; // Check if five minutes have passed since the last timestamp
    Ok((time_period_passed, false))
}

// Save data and its associated metadata (timestamp) to JSON files
pub fn save_data(data: &DataStruct, token: &str) -> io::Result<()> {
    ensure_storage_folder_exists()?; // Ensure the storage folder exists

    let json_data = serde_json::to_string(data)?; // Serialize the data to a JSON string
    let json_name = format!("{}/{}_data.json", STORAGE_FOLDER, token); // Construct the data file path
    let mut file = File::create(&json_name)?; // Create the data file
    file.write_all(json_data.as_bytes())?; // Write the JSON data to the file

    let timestamp = current_time(); // Get the current timestamp
    let metadata = Metadata { timestamp }; // Create a Metadata instance with the current timestamp

    let json_name_time = format!("{}/{}_metadata.json", STORAGE_FOLDER, token); // Construct the metadata file path
    let json_time = serde_json::to_string(&metadata)?; // Serialize the metadata to a JSON string
    let mut file_time = File::create(&json_name_time)?; // Create the metadata file
    file_time.write_all(json_time.as_bytes())?; // Write the JSON metadata to the file
    Ok(())
}

// Fetch data from a JSON file for the given token
pub fn fetch_from_json(token: &str) -> io::Result<DataStruct> {
    ensure_storage_folder_exists()?; // Ensure the storage folder exists

    let json_name = format!("{}/{}_data.json", STORAGE_FOLDER, token); // Construct the data file path
    let path = Path::new(&json_name);
    let mut file = File::open(&path)?; // Open the data file
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?; // Read the file contents into a string
    
    let data: DataStruct = serde_json::from_str(&json_data).expect("Failed to deserialize data"); // Deserialize the JSON string into DataStruct
    Ok(data)
}

// Fetch data from the API for the given token
pub async fn fetch_from_api(api_base_url: &str, token: &str) -> Result<DataStruct, Error> {
    let url = format!("{}/token/{}?stat=true&holder=true", api_base_url, token); // Construct the API URL using the base URL
    let response = reqwest::get(url).await?.json::<DataStruct>().await?; // Send a GET request and parse the JSON response
    Ok(response)
}

// Helper function to format the optional values
fn format_option(value: &Option<String>) -> String {
    value.as_deref().unwrap_or("N/A").to_string()
}

// Helper function to select a random background image based on weight
fn select_random_banner(banners: &Vec<Value>) -> &str {
    let weights: Vec<_> = banners.iter().map(|b| b["weight"].as_u64().unwrap_or(1) as u32).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let index = dist.sample(&mut rng);
    banners[index]["url"].as_str().unwrap()
}



fn format_timestamp(timestamp: u64) -> String {
    // Convert timestamp to UTC datetime
    let datetime_utc = Utc.timestamp_opt(timestamp as i64, 0).single().unwrap();

    // Convert UTC datetime to CDT
    //let datetime_cdt = datetime_utc.with_timezone(&FixedOffset::west(5 * 3600));

    let now_utc = Utc::now();
    let duration = now_utc.signed_duration_since(datetime_utc);

    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;

    let formatted_duration = if days > 0 {
        format!("{} days and {} hr ago", days, hours)
    } else if hours > 0 {
        format!("{} hr and {} min ago", hours, minutes)
    } else {
        format!("{} min ago", minutes)
    };

    format!("Last status is from {}", formatted_duration)
}




pub async fn format_data(data: DataStruct) -> CreateMessage {
    // Load the JSON template
    let template_content = fs::read_to_string("message_template.json").expect("Failed to read message template");
    let template: Value = serde_json::from_str(&template_content).expect("Failed to parse message template");

    // Ensure required fields are present in the template
    let color = 0xADD8E6; // Light blue color
    let background_images = template["background_images"].as_array().expect("Background images not found in message template");
    let background_image_url = select_random_banner(background_images);
    let author_name = template["author"]["name"].as_str().expect("Author name not found in message template");
    let author_icon_url = template["author"]["icon_url"].as_str().expect("Author icon URL not found in message template");

    let result = &data.result[0];

    // Get the current timestamp and format it
    let metadata_content = fs::read_to_string(format!("{}/{}_metadata.json", STORAGE_FOLDER, result.tick)).expect("Failed to read metadata file");
    let metadata: Metadata = serde_json::from_str(&metadata_content).expect("Failed to parse metadata");
    let formatted_timestamp = format_timestamp(metadata.timestamp);

    // Create the message payload with an embedded message
    let mut embed = CreateEmbed::new()
        .color(color)
        .image(background_image_url)
        .author(CreateEmbedAuthor::new(author_name).icon_url(author_icon_url))
        .description(formatted_timestamp) // Add the timestamp description
        .footer(CreateEmbedFooter::new("x.com/coinchimpx"));

    // Add content and footer
    let content = format!("**# Mint Status for {}**", result.tick.to_uppercase());

    // Check the state of the token
    if result.state == "unused" {
        let payload = CreateMessage::new()
            .content(content)
            .embed(embed.field(
                "Token Status",
                "This token hasn't been deployed",
                false,
            ));
        return payload;
    }

    // Format the token name for display
    let formatted_token = format!("% {} Minted", result.tick.to_uppercase());

    let progress = result.minted.parse::<f64>().expect("Not a valid f64") / result.max.parse::<f64>().expect("Not a valid f64");
    let formatted_progress = format!("{:.2}%", progress * 100.0);

    // Format large numbers
    let dec = result.dec.parse::<f64>().expect("Not a valid f64");
    let max = result.max.parse::<f64>().expect("Not a valid f64") / 10f64.powf(dec);
    let lim = result.lim.parse::<f64>().expect("Not a valid f64") / 10f64.powf(dec);
    let minted = result.minted.parse::<f64>().expect("Not a valid f64") / 10f64.powf(dec);
    let formatted_max = format_large_number(max);
    let formatted_lim = format_large_number(lim);
    let formatted_minted = format_large_number(minted);

    // Calculate the sums for the top holders
    let empty_vec = Vec::new();
    let holders = result.holder.as_ref().unwrap_or(&empty_vec);
    let sum_top_50: f64 = holders.iter().take(50).map(|h| h.amount.parse::<f64>().unwrap_or(0.0)).sum();
    let sum_top_10: f64 = holders.iter().take(10).map(|h| h.amount.parse::<f64>().unwrap_or(0.0)).sum();
    let sum_top_1: f64 = holders.iter().take(1).map(|h| h.amount.parse::<f64>().unwrap_or(0.0)).sum();

    let formatted_sum_top_50 = format_large_number(sum_top_50 / 10f64.powf(dec));
    let formatted_sum_top_10 = format_large_number(sum_top_10 / 10f64.powf(dec));
    let formatted_sum_top_1 = format_large_number(sum_top_1 / 10f64.powf(dec));

    let pre_allocation_amount = result.pre.parse::<f64>().unwrap_or(0.0) / 10f64.powf(dec);
    let pre_allocation_desc = if pre_allocation_amount == 0.0 {
        "Fair Launch".to_string()
    } else {
        format_large_number(pre_allocation_amount)
    };

    // Create the message payload with an embedded message
    embed = embed
        .field(formatted_token, formatted_progress, true)
        .field("Pre-Allocation", pre_allocation_desc, true) // Add this line
        .field("Mints", format_option(&result.mintTotal), true)
        .field("Holders", format_option(&result.holderTotal), true)
        .field("Max Supply", formatted_max, true)
        .field("Limit", formatted_lim, true)
        .field("Minted", formatted_minted, true)
        .field("Top 50 Holders", formatted_sum_top_50, true)
        .field("Top 10 Holders", formatted_sum_top_10, true)
        .field("Top Holder", formatted_sum_top_1, true);

    CreateMessage::new()
        .content(content)
        .embed(embed)
}