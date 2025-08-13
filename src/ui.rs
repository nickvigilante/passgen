mod validation;

use std::collections::HashSet;

use crate::{
    app::config::{Config, category::CodePointCategory},
    ui::validation::validate_usize,
};

#[allow(unused_imports)]
use cliclack::{confirm, input, intro, log, multiselect, note, outro, select, spinner};

pub fn opening_ui() {
    intro("Welcome to Passgen - A Secure Password Generator")
        .ok()
        .unwrap();

    let spinner = spinner();

    spinner.start("Initializing...");

    let mut config: Config = Config::new();

    spinner.stop("");

    set_password_length_ui(&mut config);

    main_menu_ui(&mut config);
}

fn set_password_length_ui(config: &mut Config) {
    let minimum_length = config.get_min_required_chars_for_active_categories();
    let password_length: usize = input(format!(
        "Enter password length (minimum {} characters):",
        minimum_length
    ))
    .placeholder(config.get_password_length().to_string().as_str())
    .validate(validate_usize)
    .validate(
        move |input: &String| match input.parse::<usize>().ok().unwrap() < minimum_length {
            true => Err(format!(
                "Password length must be at least {} characters, which is the sum of the minimum required characters for all enabled categories.",
                minimum_length
            )),
            false => Ok(()),
        },
    )
    .interact()
    .ok()
    .unwrap();

    config.set_password_length(password_length);
}

fn generate_password_ui(config: &mut Config) {
    let password = config.generate_password();
    let items: Vec<(usize, &str, &str)> = vec![
        (0, "Print", "Print the password to the console"),
        (
            1,
            "Copy to Clipboard",
            "Copy the generated password to the clipboard",
        ),
    ];
    let choices = multiselect("Password generated. Choose what to do next:")
        .items(&items)
        .initial_values(vec![1])
        .interact()
        .ok()
        .unwrap();

    if choices.contains(&0) {
        log::success(format!("Generated password: {}", password))
            .ok()
            .unwrap();
    }

    if choices.contains(&1) {
        config.save_to_clipboard(password);
        log::success("Copied password to clipboard.").ok().unwrap();
    }
}

fn main_menu_ui(config: &mut Config) {
    let items = vec![
        (0, "Generate Password", "Generate a new secure password"),
        (
            1,
            "Edit Config",
            "Edit the configuration settings, including password length, character sets, and more",
        ),
        (2, "Exit", "Exit"),
    ];

    loop {
        let choice = select("Main Menu").items(&items).interact().ok().unwrap();

        match choice {
            0 => {
                generate_password_ui(config);
            }
            1 => edit_config_ui(config),
            2 => {
                outro("Thank you for using Passgen!").ok().unwrap();
                break;
            }
            _ => continue,
        }
    }
}

fn edit_config_ui(config: &mut Config) {
    let items = vec![
        (
            0,
            "Password Length",
            "Set the length of the generated password",
        ),
        (
            1,
            "Character Sets",
            "Enable/disable characters and categories, set minimum characters per category, and so on",
        ),
        (2, "Back to Main Menu", "Return to the main menu"),
    ];

    loop {
        let choice = select("Edit Config").items(&items).interact().ok().unwrap();

        match choice {
            0 => set_password_length_ui(config),
            1 => edit_categories_ui(config),
            2 => break,
            _ => continue,
        }
    }
}

fn edit_categories_ui(config: &mut Config) {
    loop {
        let mut items: Vec<(usize, String, String)> = config
            .get_all_categories()
            .iter()
            .enumerate()
            .map(|(i, cat)| (i, cat.get_label(), cat.get_hint()))
            .collect();

        items.push((
            items.len(),
            "<- Back".to_string(),
            "Return to the previous menu".to_string(),
        ));
        let choice = select("Edit Categories")
            .items(&items)
            .interact()
            .ok()
            .unwrap();

        match choice {
            i if i < items.len() - 1 => {
                edit_single_category_ui(config, i);
            }
            i if i == items.len() - 1 => {
                break; // Back to previous menu
            }
            _ => continue,
        }
    }
}

fn edit_single_category_ui(config: &mut Config, cat_idx: usize) {
    loop {
        // Copy the values needed for validation to avoid borrowing issues
        let password_length = config.get_password_length();
        let total_min_required = config.get_min_required_chars_for_active_categories();

        if let Some(cat) = config.categories.get_mut(cat_idx) {
            let items = vec![
                (0, cat.get_toggle_label(), ""),
                (
                    1,
                    "Set Minimum Characters",
                    "Set the minimum number of characters required from this category",
                ),
                (
                    2,
                    "Toggle Individual Code Pages",
                    "Enable or disable individual code pages within this category",
                ),
                (3, "<- Back", "Return to the previous menu"),
            ];

            let choice = select(format!("Edit {}", cat.get_label()))
                .items(&items)
                .interact()
                .ok()
                .unwrap();

            match choice {
                0 => {
                    if !cat.is_enabled()
                        && password_length < total_min_required + cat.get_min_required_chars()
                    {
                        log::error(format!(
                            "Enabling this category requires a password length of at least {} characters.",
                            total_min_required
                        ))
                        .ok()
                        .unwrap();
                    } else {
                        cat.toggle_enabled();
                    }
                }
                1 => edit_min_characters_for_category_ui(cat, password_length, total_min_required),
                2 => toggle_code_pages_ui(config, cat_idx),
                3 => break, // Back to previous menu
                _ => continue,
            }
        }
    }
}

fn edit_min_characters_for_category_ui(
    cat: &mut CodePointCategory,
    password_length: usize,
    total_min_required: usize,
) {
    let max_allowed_for_cat =
        match password_length < total_min_required - cat.get_min_required_chars() {
            true => 0,
            false => password_length - total_min_required + cat.get_min_required_chars(),
        };
    let is_category_enabled = cat.is_enabled();

    let min_chars: usize = input(format!(
        "Enter minimum required length for this category (between 0 and {} characters):",
        max_allowed_for_cat,
    ))
    .placeholder(cat.get_min_required_chars().to_string().as_str())
    .validate(validate_usize)
    .validate({
        move |input: &String| match input.parse::<usize>().ok().unwrap() <= max_allowed_for_cat
            && is_category_enabled
        {
            true => Ok(()),
            false => Err(format!(
                "Value must be between 0 and {}",
                max_allowed_for_cat
            )),
        }
    })
    .interact()
    .ok()
    .unwrap();
    cat.set_min_required_chars(min_chars);
}

fn toggle_code_pages_ui(config: &mut Config, cat_idx: usize) {
    if let Some(cat) = config.categories.get_mut(cat_idx) {
        let orig_indices = cat
            .get_code_points()
            .iter()
            .enumerate()
            .filter_map(|(i, cpc)| cpc.is_enabled().then(|| i))
            .collect::<Vec<usize>>();

        let items: Vec<(usize, String, &str)> = cat
            .get_code_points()
            .iter()
            .enumerate()
            .map(|(i, cp)| (i, cp.get_label(), ""))
            .collect();

        let new_indices = multiselect("Select code pages to enable/disable (use Space to toggle):")
            .items(&items)
            .initial_values(orig_indices.clone())
            .max_rows(30)
            .filter_mode()
            .interact()
            .ok()
            .unwrap();

        let new_hashset: HashSet<usize> = new_indices.iter().cloned().collect();
        let orig_hashset: HashSet<usize> = orig_indices.iter().cloned().collect();

        let indices_to_enable = new_hashset
            .difference(&orig_hashset)
            .cloned()
            .collect::<Vec<usize>>();

        let indices_to_disable = orig_hashset
            .difference(&new_hashset)
            .cloned()
            .collect::<Vec<usize>>();

        for &index in &indices_to_enable {
            if let Some(cpc) = cat.code_points.get_mut(index) {
                cpc.set_enabled(true);
            }
        }

        for &index in &indices_to_disable {
            if let Some(cpc) = cat.code_points.get_mut(index) {
                cpc.set_enabled(false);
            }
        }
    }
}
