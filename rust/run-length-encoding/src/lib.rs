pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut last_character = String::new();
    let mut character_number = 1;

    for character in source.chars() {
        let character_as_string = character.to_string();

        if last_character.is_empty() {
            last_character = character_as_string;
            continue;
        }

        if character_as_string == last_character {
            character_number += 1;
        } else if character_number > 1 {
            result += &(character_number.to_string() + &last_character);
            character_number = 1;
            last_character = character_as_string;
        } else {
            result += &last_character;
            last_character = character_as_string;
        }
    }

    if !last_character.is_empty() && character_number > 1{
        result += &(character_number.to_string() + &last_character);
    } else if !last_character.is_empty() {
        result += &last_character;
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut number = String::new();

    for character in source.chars() {
        if character.is_digit(10) {
            number += &character.to_string();
            continue;
        }

        if !number.is_empty() {
            result += &character.to_string().repeat(number.parse::<usize>().unwrap());
        } else {
            result += &character.to_string();
        }

        number = String::new();
    }

    result
}
