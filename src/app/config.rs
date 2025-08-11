pub mod category;
pub mod code_point;

use rand::seq::SliceRandom;

use passgen::{generate_random_chars_from_char_vec, generate_rng};

use self::category::{CodePointCategory, generate_code_point_categories};

pub struct Config {
    password_length: usize,
    categories: Vec<CodePointCategory>,
}

impl Config {
    pub fn new(password_length: usize, include_extended: bool) -> Self {
        let categories: Vec<CodePointCategory> = generate_code_point_categories(include_extended);
        if categories.iter().map(|c| c.get_min_chars()).sum::<usize>() > password_length {
            panic!(
                "The sum of minimum characters from all categories must be less than or equal to the password length."
            );
        }
        Config {
            password_length,
            categories,
        }
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
                        cat.get_min_chars(),
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
}
