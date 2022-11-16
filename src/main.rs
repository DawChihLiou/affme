use affme::affirm;
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
    let output = affirm(&name);

    println!("{}", output);
}
