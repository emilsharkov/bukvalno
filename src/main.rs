use bukvalno::{generate_ascii_art, cli::Config};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse_args_to_config()?;
    let ascii_art = generate_ascii_art(&config)?;
    print!("{}", ascii_art);
    Ok(())
}