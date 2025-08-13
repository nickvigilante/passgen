pub fn validate_usize(input: &String) -> Result<(), String> {
    match input.parse::<usize>() {
        Ok(length) if length > 0 && length <= usize::MAX => Ok(()),
        _ => Err(format!(
            "Please enter a valid number between 1 and {}.",
            usize::MAX
        )),
    }
}
