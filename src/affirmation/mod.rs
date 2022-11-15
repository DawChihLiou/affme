use crate::random;

pub struct Affirmation {
    affirmations: Vec<String>,
}

impl Affirmation {
    pub fn new() -> Self {
        let affirmations = [
            "You're beautiful",
            "You're awesome",
            "You're wonderful",
            "You've got this",
            "You can do all things",
            "Go get it",
        ]
        .map(String::from)
        .to_vec();

        Affirmation { affirmations }
    }

    pub fn random(&self) -> String {
        random::pick(&self.affirmations).unwrap_or("You're the best".to_string())
    }
}
