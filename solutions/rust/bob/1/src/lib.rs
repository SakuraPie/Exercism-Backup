pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if is_all_upper(message) && message.ends_with("?") {
        return "Calm down, I know what I'm doing!";
    }
    if message.ends_with("?") {
        return "Sure.";
    }
    if is_all_upper(message) {
        return "Whoa, chill out!";
    }
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    "Whatever."
}

fn is_all_upper(s: &str) -> bool {
    s.to_uppercase() == s && s.chars().any(|c| c.is_alphabetic())
}
