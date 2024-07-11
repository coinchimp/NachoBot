// result_struct.rs
use serde::{Serialize, Deserialize};

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
    pub holderTotal: String,
    pub transferTotal: String,
    pub mintTotal: String,
}
