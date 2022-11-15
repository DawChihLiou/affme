mod affirmation;
mod args;
mod formatter;
mod random;

use affirmation::Affirmation;

use clap::Parser;

/// ✏️  Simple program show affirmations to someone
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
    let output = formatter::format(&affirmation, &name);

    println!("{}", output);
}
