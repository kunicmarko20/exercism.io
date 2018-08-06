const CHARACTERS_REGULAR: &'static str = "abcdefghijklmnopqrstuvwxyz";
const CHARACTERS_REVERSED: &'static str = "zyxwvutsrqponmlkjihgfedcba";

pub fn encode(plain: &str) -> String {
    let mut encoded = String::new();

    for character in plain.to_lowercase().chars() {
        match CHARACTERS_REGULAR.find(character) {
            Some(position) => encoded += &CHARACTERS_REVERSED[position..position+1],
            None => {
                if let Some(digit) = character.to_digit(10) {
                    encoded += &digit.to_string();
                } else {
                    continue;
                }
            }
        }

        if should_add_space_to(&encoded) {
            encoded += " ";
        }
    }

    encoded.trim().to_string()
}

fn should_add_space_to(encoded: &String) -> bool {
    get_count_of_valid_characters(encoded) % 5 == 0 
}

fn get_count_of_valid_characters(encoded: &String) -> usize {
    encoded.replace(" ", "").chars().count()
}
/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut decoded = String::new();

    for character in cipher.chars() {
        match CHARACTERS_REVERSED.find(character) {
            Some(position) => decoded += &CHARACTERS_REGULAR[position..position+1],
            None =>  {
                if let Some(digit) = character.to_digit(10) {
                    decoded += &digit.to_string();
                }
            }
        }
    }

    decoded
}
