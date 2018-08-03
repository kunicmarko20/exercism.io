/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = remove_space_in_string(code);

    if code.chars().count() < 2 {
        return false;
    }

    let mut doubled_digits = Vec::new();

    for (index, character) in code.chars().rev().enumerate() {
        match get_digit_based_on_reminder(index, character.to_digit(10)) {
            None => return false,
            Some(digit) => doubled_digits.push(digit)
        }
    }

    doubled_digits.iter().sum::<u32>() % 10 == 0
}

fn remove_space_in_string(code: &str) -> String
{
    code.replace(" ", "")
}

fn get_digit_based_on_reminder(index: usize, option_digit: Option<u32>) -> Option<u32>
{
    match option_digit {
        None => return None,
        Some(digit) => {   
            if index % 2 != 0 {
                let doubled_digit = digit + digit;

                if doubled_digit > 9 {
                    return Some(doubled_digit - 9);
                }
        
                return Some(doubled_digit);
            }
            
            return Some(digit);
        }
    }
}