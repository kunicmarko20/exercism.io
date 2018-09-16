pub fn is_armstrong_number(number: u32) -> bool {
    let number_as_string = number.to_string();
    let length: u32 = number_as_string.len() as u32;
    let mut sum = 0;

    for character in number_as_string.chars() {
        sum += character_to_integer(character).pow(length);
    }

    sum == number
}

fn character_to_integer(character: char) -> u32
{
    character.to_string().parse::<u32>().unwrap()
}
