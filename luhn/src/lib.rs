/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = clean_string(code);

    if code.chars().count() < 2 {
        return false;
    }

    let mut doubled_digits = Vec::new();

    for (index, character) in code.chars().rev().enumerate() {
        if !character.is_digit(10) {
            return false;
        }

        doubled_digits.push(get_digit(index, character));
    }

    doubled_digits.iter().sum::<u32>() % 10 == 0
}

fn clean_string(code: &str) -> String
{
    code.replace(" ", "")
}

fn character_to_integer(character: char) -> u32
{
    character.to_string().parse::<u32>().unwrap()
}

fn get_digit(index: usize, character: char) -> u32
{
    let character_as_integer = character_to_integer(character);

    if index % 2 != 0 {
        if (character_as_integer + character_as_integer) > 9 {
            return character_as_integer + character_as_integer - 9;
        }
 
        return character_as_integer + character_as_integer;
    }
    
    character_as_integer
}