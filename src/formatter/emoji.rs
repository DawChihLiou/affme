use crate::random;

#[derive(Debug)]
pub struct Emoji {
    emojis: Vec<String>,
}

impl Emoji {
    pub fn new() -> Self {
        let emojis: Vec<String> = vec!["😍", "😎", "🧸", "😉", "👍", "💪", "✨"]
            .into_iter()
            .map(String::from)
            .collect();

        Emoji { emojis }
    }

    pub fn random(&self) -> String {
        let emoji = random::pick(&self.emojis);

        match emoji {
            Some(value) => value,
            None => "💙".to_string(),
        }
    }
}
