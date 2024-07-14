use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Holder {
    pub address: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultStruct {
    pub tick: String,
    pub max: String,
    pub lim: String,
    pub pre: String,
    pub to: String,
    pub dec: String,
    pub minted: String,
    pub opScoreAdd: String,
    pub opScoreMod: String,
    pub state: String,
    pub hashRev: String,
    pub mtsAdd: String,
    pub holderTotal: Option<String>,
    pub transferTotal: Option<String>,
    pub mintTotal: Option<String>,
    pub holder: Option<Vec<Holder>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStruct {
    pub message: String,
    pub result: Vec<ResultStruct>,
}
