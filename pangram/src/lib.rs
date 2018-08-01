use std::collections::HashSet;

const ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                    's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

pub fn is_pangram(sentence: &str) -> bool {
    let mut characters: HashSet<char> = ASCII_LOWER.iter().cloned().collect();

    for character in sentence.chars() {
        let to_lowercase = character.to_lowercase().collect::<Vec<_>>();
        let lowercase_character = to_lowercase.first().unwrap();
        if characters.contains(lowercase_character) {
            characters.remove(lowercase_character);
        }
    }

    characters.is_empty()
}
