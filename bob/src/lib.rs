pub fn reply(message: &str) -> &str {
    let message = &message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    if is_yelling(message) && is_question(message) {
        return "Calm down, I know what I'm doing!";
    }

    if is_yelling(message) {
        return "Whoa, chill out!";
    }

    if is_question(message) {
        return "Sure.";
    }

    "Whatever."
}

fn is_yelling(message: &str) -> bool
{
    message == message.to_string().to_uppercase() && message != message.to_string().to_lowercase()
}

fn is_question(message: &str) -> bool
{
    message.chars().last().unwrap() == '?'
}