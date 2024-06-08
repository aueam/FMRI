/// Checks if inserted string contains @
pub fn check_character_collision(string: &str) -> Result<(), String> {
    for char in string.chars() {
        if char == '@' {
            return Err(String::from("String cannot contain char \'@\'"));
        }
    }
    Ok(())
}

/// Removes first and last characters if it is inputted character
pub fn remove_first_and_last_characters(string: &str, character: char) -> String {
    string
        .trim_start_matches(character)
        .trim_end_matches(character)
        .to_owned()
}
