// cliclack::Input::validate requires exactly `Fn(&String) -> _` (its
// `Validate<String>` bound) -- &str doesn't satisfy it without a wrapper
// closure at every call site, so the usual &String -> &str lint doesn't
// apply here.
#[allow(clippy::ptr_arg)]
pub fn validate_usize(input: &String) -> Result<(), String> {
    match input.parse::<usize>() {
        Ok(length) if length > 0 => Ok(()),
        _ => Err(format!(
            "Please enter a valid number between 1 and {}.",
            usize::MAX
        )),
    }
}
