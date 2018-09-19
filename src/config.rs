#[derive(Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub height: f32,
    pub width: f32
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            height: 100.0,
            width: 100.0
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GameConfig {
    pub arena: ArenaConfig
}