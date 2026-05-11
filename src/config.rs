use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WindowSize {
    S,
    M,
    L,
}

impl Default for WindowSize {
    fn default() -> Self {
        WindowSize::M
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProfileConfig {
    #[serde(default = "default_work_duration_secs")]
    pub work_duration_secs: u32,
    #[serde(default = "default_short_break_secs")]
    pub short_break_secs: u32,
    #[serde(default = "default_long_break_secs")]
    pub long_break_secs: Option<u32>,
    #[serde(default = "default_cycles_before_long_break")]
    pub cycles_before_long_break: Option<u32>,
    #[serde(default = "default_work_color")]
    pub work_color: String,
    #[serde(default = "default_short_break_color")]
    pub short_break_color: String,
    #[serde(default = "default_long_break_color")]
    pub long_break_color: String,
}

fn default_work_duration_secs() -> u32 { 1500 }
fn default_short_break_secs() -> u32 { 300 }
fn default_long_break_secs() -> Option<u32> { Some(900) }
fn default_cycles_before_long_break() -> Option<u32> { Some(4) }
fn default_work_color() -> String { "#E05C5C".to_string() }
fn default_short_break_color() -> String { "#5CB85C".to_string() }
fn default_long_break_color() -> String { "#5C9BE0".to_string() }

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            work_duration_secs: default_work_duration_secs(),
            short_break_secs: default_short_break_secs(),
            long_break_secs: default_long_break_secs(),
            cycles_before_long_break: default_cycles_before_long_break(),
            work_color: default_work_color(),
            short_break_color: default_short_break_color(),
            long_break_color: default_long_break_color(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GlobalConfig {
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default = "default_sound_enabled")]
    pub sound_enabled: bool,
    #[serde(default = "default_volume")]
    pub volume: f32,
    #[serde(default)]
    pub window_size: WindowSize,
    #[serde(default = "default_always_on_top")]
    pub always_on_top: bool,
    #[serde(default)]
    pub window_x: Option<f32>,
    #[serde(default)]
    pub window_y: Option<f32>,
    #[serde(default = "default_active_profile_name")]
    pub active_profile_name: String,
}

fn default_opacity() -> f32 { 0.30 }
fn default_sound_enabled() -> bool { true }
fn default_volume() -> f32 { 0.70 }
fn default_always_on_top() -> bool { true }
fn default_active_profile_name() -> String { "classic".to_string() }

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            opacity: default_opacity(),
            sound_enabled: default_sound_enabled(),
            volume: default_volume(),
            window_size: WindowSize::default(),
            always_on_top: default_always_on_top(),
            window_x: None,
            window_y: None,
            active_profile_name: default_active_profile_name(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    #[serde(default)]
    pub global: GlobalConfig,
    #[serde(default = "default_profiles")]
    pub profiles: HashMap<String, ProfileConfig>,
}

fn default_profiles() -> HashMap<String, ProfileConfig> {
    let mut profiles = HashMap::new();
    profiles.insert("classic".to_string(), ProfileConfig::default());
    
    let mut no_long_break = ProfileConfig::default();
    no_long_break.long_break_secs = None;
    no_long_break.cycles_before_long_break = None;
    profiles.insert("no_long_break".to_string(), no_long_break);
    
    profiles
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            global: GlobalConfig::default(),
            profiles: default_profiles(),
        }
    }
}

impl AppConfig {
    fn config_path() -> PathBuf {
        let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        path.pop();
        path.push("config.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str(&contents) {
                return config;
            }
        }
        let default_config = Self::default();
        default_config.save();
        default_config
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Ok(contents) = toml::to_string_pretty(self) {
            let _ = fs::write(&path, contents);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_roundtrip() {
        let original = AppConfig::default();
        let serialized = toml::to_string_pretty(&original).expect("Failed to serialize");
        let deserialized: AppConfig = toml::from_str(&serialized).expect("Failed to deserialize");

        // Comprova globals
        assert_eq!(original.global.opacity, deserialized.global.opacity);
        assert_eq!(original.global.sound_enabled, deserialized.global.sound_enabled);
        assert_eq!(original.global.active_profile_name, deserialized.global.active_profile_name);

        // Comprova que els dos perfils hi són
        assert!(deserialized.profiles.contains_key("classic"));
        assert!(deserialized.profiles.contains_key("no_long_break"));

        // Comprova valors del perfil classic
        let classic = deserialized.profiles.get("classic").unwrap();
        assert_eq!(classic.work_duration_secs, 1500);
        assert_eq!(classic.short_break_secs, 300);
        assert_eq!(classic.long_break_secs, Some(900));
    }
}
