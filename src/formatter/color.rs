use crate::random;

#[derive(Debug)]
pub struct Color {
    colors: Vec<String>,
}

impl Color {
    pub fn new() -> Self {
        let colors: Vec<String> = ["red", "blue", "green", "yellow", "magenta", "cyan"]
            .map(String::from)
            .to_vec();

        Color { colors }
    }

    pub fn random(&self) -> String {
        random::pick(&self.colors).unwrap_or("yellow".to_string())
    }
}
