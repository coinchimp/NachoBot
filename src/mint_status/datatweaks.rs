use crate::DataStruct; // Import the DataStruct from the current crate
use crate::imports::*; // Import everything from the imports module
use serde_json::Value;
use std::fs;

const STORAGE_FOLDER: &str = "data_storage"; // Define a constant for the storage folder name

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

    let five_minutes_passed = current_time > metadata.timestamp + 300; // Check if five minutes have passed since the last timestamp
    Ok((five_minutes_passed, false))
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


// Format the fetched data into a message to be sent
pub async fn format_data(data: DataStruct) -> CreateMessage {
    // Load the JSON template
    let template_content = fs::read_to_string("message_template.json").expect("Failed to read message template");
    let template: Value = serde_json::from_str(&template_content).expect("Failed to parse message template");

    // Ensure required fields are present in the template
    let color = template["color"].as_u64().expect("Color not found in message template") as u32;
    let background_image_url = template["background_image_url"].as_str().expect("Background image URL not found in message template");
    let author_name = template["author"]["name"].as_str().expect("Author name not found in message template");
    let author_icon_url = template["author"]["icon_url"].as_str().expect("Author icon URL not found in message template");

    let result = &data.result[0];
    let progress = result.minted.parse::<f64>().expect("Not a valid f64") / result.max.parse::<f64>().expect("Not a valid f64"); // Calculate the progress percentage
    let formatted_progress = format!("{:.2}%", progress * 100.0); // Format the progress as a percentage
    
    // Format the token name for display
    let mut chars = result.tick.chars();
    let first_char = chars.next().unwrap().to_uppercase().collect::<String>();
    let remaining_chars = chars.as_str().to_lowercase();
    let formatted_token = format!("Percent of {}{} Minted", first_char, remaining_chars);
    
    // Create the message payload with an embedded message
    let payload = CreateMessage::new().embed(CreateEmbed::new()
        .color(color)
        .image(background_image_url)
        .field(formatted_token, formatted_progress, false)
        .author(CreateEmbedAuthor::new(author_name)
            .icon_url(author_icon_url)
        ));
    payload
}
