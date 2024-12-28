use bukvalno::{ascii, cli};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = cli::Config::parse_args_to_config()?;
    let ascii_art = ascii::generate_ascii_art(&config)?;
    print!("{}", ascii_art);
    Ok(())
}