mod affirmation;
mod formatter;
mod random;

use affirmation::Affirmation;
use formatter::format;

use clap::Parser;

/// ✏️  Simple program to show affirmations to someone
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to affirm
    #[arg(short, long)]
    name: String,
}

fn main() {
    let name = Args::parse().name;
    let affirmation = Affirmation::new().random();
    let output = format(&affirmation, &name);

    println!("{}", output);
}
