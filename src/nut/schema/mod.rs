#[derive(Serialize, Deserialize, Debug)]
pub struct Migration {
    pub version: String,
    pub name: String,
    pub up: String,
    pub down: String,
}
