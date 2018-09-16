
pub fn check(candidate: &str) -> bool {
    let mut result: Vec<String> = Vec::new();
    let allowed_to_repeat = vec![" ", "-"];

    for character in candidate.chars() {
        let character_as_string = character.to_string().to_lowercase();

        if !allowed_to_repeat.contains(&&character_as_string[..]) && result.contains(&character_as_string) {
            return false;
        }

        result.push(character_as_string);
    }

    true
}
