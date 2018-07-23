pub fn build_proverb(list: Vec<&str>) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut result: Vec<String> = Vec::new();
    let list_length = list.len() - 1;
    let mut current = String::new();

    for (index, item) in list.iter().enumerate() {
        if !&current.is_empty() {
            result.push(format!("{}{}", current, format!("the {} was lost.", item)));
        }

        if !is_last_element(list_length, index) {
            current = format!("For want of a {} ", item);
        } else {
            result.push(format!("And all for the want of a {}.", list[0]));
        }
    }

    result.join("\n")
}

fn is_last_element(list_length: usize, index: usize) -> bool
{
    list_length == index
}