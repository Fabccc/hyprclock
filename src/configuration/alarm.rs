// src/configuration/alarm.rs
// github.com/cvusmo/hyprclock

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlarmsConfig {
    pub alarms: Vec<AlarmConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlarmConfig {
    /// The duration in seconds
    duration: u64,
    /// Alarm's name, if empty, fallback to formatted time
    name: Option<String>,
}

impl AlarmsConfig {
    pub fn new() -> Self {
        AlarmsConfig {
            alarms: vec![AlarmConfig::from_minutes(5), AlarmConfig::from_minutes(10)],
        }
    }
}

impl AlarmConfig {
    pub fn new() -> Self {
        AlarmConfig {
            duration: 60,
            name: None,
        }
    }

    pub fn from_seconds(duration: u64) -> Self {
        AlarmConfig {
            duration,
            name: None,
        }
    }

    pub fn from_minutes(duration: u64) -> Self {
        Self::from_seconds(duration * 60)
    }

    pub fn get_name(&self) -> String {
        let name_cloned = self.name.clone();
        match self.duration {
            0_u64..=59_u64 => name_cloned.unwrap_or(format!("{} seconds", self.duration)),
            60_u64..=119_u64 => name_cloned.unwrap_or(format!("{} minute", self.duration / 60)),
            _ => name_cloned.unwrap_or(format!("{} minutes", self.duration / 60)),
        }
    }
}
