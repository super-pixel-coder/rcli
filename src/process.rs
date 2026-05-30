use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for record in reader.deserialize() {
        let player: Player = record?;
        ret.push(player);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, json)?;
    Ok(())
}
