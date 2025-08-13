pub mod category;
pub mod code_point;

use arboard::Clipboard;

use rand::seq::SliceRandom;

use passgen::{generate_random_chars_from_char_vec, generate_rng};

use self::category::{CodePointCategory, generate_code_point_categories};

pub struct Config {
    password_length: usize,
    clipboard: Clipboard,
    pub categories: Vec<CodePointCategory>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            password_length: 128,
            clipboard: Clipboard::new().unwrap(),
            categories: generate_code_point_categories(),
        }
    }

    pub fn get_all_categories(&self) -> &Vec<CodePointCategory> {
        &self.categories
    }

    fn get_active_categories(&self) -> Vec<&CodePointCategory> {
        self.categories
            .iter()
            .filter(|cat| cat.is_enabled() && cat.get_active_code_points().len() > 0)
            .collect()
    }

    fn get_active_chars_for_active_categories(&self) -> Vec<char> {
        self.get_active_categories()
            .iter()
            .flat_map(|cat| cat.get_active_chars())
            .collect()
    }

    pub fn generate_password(&self) -> String {
        let mut rng = generate_rng();
        let mut result: Vec<char> = self
            .get_active_categories()
            .iter()
            .filter_map(|cat| {
                (cat.get_active_code_points().len() > 0).then(|| {
                    generate_random_chars_from_char_vec(
                        cat.get_active_chars(),
                        cat.get_min_required_chars(),
                        &mut rng,
                    )
                })
            })
            .flatten()
            .collect();
        let remaining_length = self.password_length - result.len();
        if remaining_length > 0 {
            result.append(&mut generate_random_chars_from_char_vec(
                self.get_active_chars_for_active_categories(),
                remaining_length,
                &mut rng,
            ));
        }
        result.shuffle(&mut rng);
        result.into_iter().collect::<String>()
    }

    pub fn get_password_length(&self) -> usize {
        self.password_length
    }

    pub fn set_password_length(&mut self, length: usize) {
        self.password_length = length;
    }

    pub fn save_to_clipboard(&mut self, password: String) {
        self.clipboard.set_text(password).unwrap();
    }

    pub fn get_min_required_chars_for_active_categories(&self) -> usize {
        self.get_active_categories()
            .iter()
            .map(|cat| cat.get_min_required_chars())
            .sum()
    }
}
