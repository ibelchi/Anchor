pub struct Config {
    pub sound_enabled: bool,
    pub volume: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sound_enabled: true,
            volume: 1.0,
        }
    }
}
