// Importing required modules and structures from the crate
use crate::DataStruct;
use crate::imports::*;

// Metadata struct to store a timestamp for serialization and deserialization
#[derive(Serialize, Deserialize)]
struct Metadata {
    timestamp: u64,
}

// Function to get the current system time in seconds since UNIX_EPOCH
pub fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

// Function to check if 5 minutes have passed since the last recorded timestamp
pub fn check_time() -> io::Result<bool> {
    let current_time = current_time();

    // Open the metadata file for reading
    let mut file = OpenOptions::new().read(true).open("metadata.json")?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    
    // Deserialize the metadata from the JSON string
    let metadata: Metadata = serde_json::from_str(&json)?;

    // Check if 5 minutes (300 seconds) have passed
    let five_minutes_passed = current_time > metadata.timestamp + 300;
    println!("\nOLD: {}\nNEW: {}\nNEW DATA: {}", metadata.timestamp, current_time, five_minutes_passed);
    Ok(five_minutes_passed)
}

// Function to save data into a JSON file and update the metadata with the current timestamp
pub fn save_data(data: &DataStruct) -> io::Result<()> {
    // Serialize the data struct to a JSON string
    let json_data = serde_json::to_string(data)?;
    let mut file = File::create("data.json")?;
    file.write_all(json_data.as_bytes())?;

    // Get the current timestamp and create metadata
    let timestamp = current_time();
    let metadata = Metadata { timestamp };

    // Serialize the metadata and save it to the metadata file
    let json_time = serde_json::to_string(&metadata)?;
    let mut file_time = File::create("metadata.json")?;
    file_time.write_all(json_time.as_bytes())?;
    Ok(())
}

// Function to fetch data from a JSON file and deserialize it into a DataStruct
pub fn fetch_from_json(file_path: &str) -> io::Result<DataStruct> {
    let path = Path::new(file_path);
    let mut file = File::open(&path)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;
    
    // Deserialize the JSON string into a DataStruct
    let data: DataStruct = serde_json::from_str(&json_data).expect("Failed to deserialize data");
    Ok(data)
}

// Asynchronous function to fetch data from an API and deserialize it into a DataStruct
pub async fn fetch_from_api() -> Result<DataStruct, Error> {
    let url = String::from("https://tn11api.kasplex.org/v1/krc20/token/NACHO");
    let response = reqwest::get(url).await?.json::<DataStruct>().await?;
    Ok(response)
}

// Asynchronous function to format data and create a message with an embedded structure
pub async fn format_data(data: DataStruct) -> CreateMessage {
    let result = &data.result[0];
    let progress = result.minted.parse::<f64>().expect("Not a valid f64") / result.max.parse::<f64>().expect("Not a valid f64");
    
    let formatted_progress = format!("{:.2}%", progress * 100.0);
    
    // Creating an embedded message with formatted data
    let payload = CreateMessage::new().embed(CreateEmbed::new()
        .color(7391162)
        .field("Percent of Nacho Minted", formatted_progress, false)
        .author(CreateEmbedAuthor::new("Temp")
            .name("Nacho the 𐤊at")
        ));
    payload
}
