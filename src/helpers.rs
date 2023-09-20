/// Checks if inserted string contains one of "bad" characters
pub fn check_character_collision(string: &String) {
    for char in string.chars() {
        if char == '@' {
            panic!("String cannot contain char \'@\'")
        }
    }
}

/// Removes first and last characters if it is inputted character
pub fn remove_first_and_last_characters<'a>(string: &String, character: char) -> String {
    string.trim_start_matches(character).trim_end_matches(character).to_owned()
}