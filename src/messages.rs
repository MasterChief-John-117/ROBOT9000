pub fn clean(message: &str) -> String {
    let mut clean = String::new();
    let approved_chars = String::from("abcdefghijklmnopqrstuvwxyz1234567890");

    for c in message.to_lowercase().chars() {
        if approved_chars.contains(c) {
            clean.push(c);
        }
    }

    return clean;
}