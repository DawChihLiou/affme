mod affirmation;
mod formatter;
mod random;

use affirmation::Affirmation;
use formatter::format;

pub fn affirm(name: &str) -> String {
    let affirmation = Affirmation::new().random();

    format(&affirmation, &name)
}
