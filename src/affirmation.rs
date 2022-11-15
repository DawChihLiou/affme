use crate::random;

#[derive(Debug)]
pub struct Affirmation<'a> {
    affirmations: [&'a str; 6],
}

impl<'a> Affirmation<'a> {
    pub fn new() -> Self {
        let affirmations = [
            "You're beautiful",
            "You're awesome",
            "You're wonderful",
            "You've got this",
            "You can do all things",
            "Go get it",
        ];

        Affirmation { affirmations }
    }

    pub fn random(&self) -> &'a str {
        random::pick(&self.affirmations)
    }
}
