pub mod data;

use crate::app::config::code_point::{CodePointConfig, Filterable};

pub use self::data::generate_code_point_categories;

#[allow(dead_code)]
pub struct CodePointCategory {
    name: String,
    enabled: bool,
    filters: Box<dyn Filterable>,
    min_characters: usize,
    pub code_points: Vec<CodePointConfig>,
}

impl CodePointCategory {
    pub fn new(
        name: &str,
        enabled: bool,
        filters: Box<dyn Filterable>,
        min_characters: usize,
    ) -> Self {
        Self {
            name: name.to_string(),
            enabled,
            filters,
            min_characters,
            code_points: Vec::new(),
        }
    }

    pub fn get_min_required_chars(&self) -> usize {
        self.min_characters
    }

    pub fn set_min_required_chars(&mut self, min_chars: usize) {
        self.min_characters = min_chars;
    }

    pub fn get_code_points(&self) -> Vec<CodePointConfig> {
        self.code_points.clone()
    }

    pub fn get_active_code_points(&self) -> Vec<CodePointConfig> {
        self.code_points
            .iter()
            .filter(|cpc| cpc.is_enabled())
            .cloned()
            .collect()
    }

    pub fn get_code_point_count(&self) -> usize {
        self.code_points.len()
    }

    pub fn get_active_code_point_count(&self) -> usize {
        self.get_active_code_points().len()
    }

    pub fn get_active_chars(&self) -> Vec<char> {
        self.get_active_code_points()
            .iter()
            .map(|cpc| cpc.get_char())
            .collect()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn get_pretty_enabled(&self) -> String {
        match self.enabled {
            true => "Enabled".to_string(),
            false => "Disabled".to_string(),
        }
    }

    pub fn get_label(&self) -> String {
        format!(
            "{} - {} - {}/{} Enabled Code Points - Minimum Characters Required: {}",
            self.name,
            self.get_pretty_enabled(),
            self.get_active_code_point_count(),
            self.get_code_point_count(),
            self.min_characters
        )
    }

    pub fn get_hint(&self) -> String {
        let max_length = 20;
        if self.get_active_chars().len() > max_length {
            format!(
                "{}...",
                &self.get_active_chars()[..max_length]
                    .iter()
                    .collect::<String>()
            )
        } else {
            self.get_active_chars().iter().collect::<String>()
        }
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn get_toggle_label(&self) -> &str {
        match self.enabled {
            true => "Disable Category",
            false => "Enable Category",
        }
    }
}
