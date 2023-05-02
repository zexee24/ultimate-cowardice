use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ChampPool{
    pub top: Vec<String>,
    pub jng: Vec<String>,
    pub mid: Vec<String>,
    pub bot: Vec<String>,
    pub sup: Vec<String>,
}
