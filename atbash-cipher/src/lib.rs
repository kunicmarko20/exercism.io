const CHARACTERS_REGULAR: &'static str = "abcdefghijklmnopqrstuvwxyz0123456789";
const CHARACTERS_REVERSED: &'static str = "zyxwvutsrqponmlkjihgfedcba0123456789";

pub fn encode(plain: &str) -> String {
    transform_input(plain, CHARACTERS_REGULAR, CHARACTERS_REVERSED)
        .chars()
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>() + " ")
        .collect::<String>()
        .trim()
        .to_string()
}

fn transform_input(input: &str, find_in_string: &'static str, string_for_replace: &'static str) -> String {
    input.to_lowercase().chars().map(|character| {
        if let Some(position) = find_in_string.find(character) {
            return &string_for_replace[position..position+1];
        }
 
        return "";
    }).collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(plain: &str) -> String {
    transform_input(plain, CHARACTERS_REVERSED, CHARACTERS_REGULAR)
}
