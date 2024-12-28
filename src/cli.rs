use clap::{Arg, Command};
use std::error::Error;

use crate::charsets;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub scale: f32,
    pub charset_label: String,
    pub invert: bool,
}

impl Config {
    pub fn parse_args_to_config() -> Result<Self, Box<dyn Error>> {
        let command = Self::get_command();
        let mut config = Self::get_args_from_matches(command)?;
        config.validate_args()?;
        Ok(config)
    }

    fn get_command() -> Command {
        Command::new("bukvalno")
            .author("Emil Sharkov <emosharkov@gmail.com>")
            .about("Image to ASCII Art Generator")
            .arg(
                Arg::new("file")
                    .short('f')
                    .long("file")
                    .value_name("FILE")
                    .help("Sets the input image file")
                    .required(true),
            )
            .arg(
                Arg::new("scale")
                    .short('s')
                    .long("scale")
                    .value_name("SCALE")
                    .help("Sets the scale down factor")
                    .default_value("1.0"),
            )
            .arg(
                Arg::new("charset")
                    .short('c')
                    .long("charset")
                    .value_name("CHARSET")
                    .help("Sets the charset ie. english, chinese, japanese, braille, circles, blocks")
                    .default_value("english"),
            )
            .arg(
                Arg::new("invert")
                    .short('i')
                    .long("invert")
                    .value_name("INVERT")
                    .help("Inverts the dark and light colors of the image")
                    .default_value("false"),
            )
    }

    fn get_args_from_matches(command: Command) -> Result<Self, Box<dyn Error>> {
        let matches = command.get_matches();

        let filename = matches
            .get_one::<String>("file")
            .ok_or("Filename is required")?
            .to_string();
        let scale = matches
            .get_one::<String>("scale")
            .ok_or("Scale has a default value")?
            .parse::<f32>()
            .map_err(|_| "Scale must be a valid number")?;
        let charset_label = matches
            .get_one::<String>("charset")
            .ok_or("Charset has a default value")?
            .to_string();
        let invert = matches
            .get_one::<String>("invert")
            .ok_or("Invert has a default value")?
            .parse::<bool>()
            .map_err(|_| "Invert must be a valid boolean")?;

        Ok(Self {
            filename,
            scale,
            charset_label,
            invert,
        })
    }

    fn validate_args(&mut self) -> Result<(), Box<dyn Error>> {
        Self::validate_scale(self.scale)?;
        Self::validate_charset(&self.charset_label)?;
        Ok(())
    }

    fn validate_scale(scale: f32) -> Result<(), Box<dyn Error>> {
        if scale <= 0.0 {
            return Err("Scale must be greater than 0".into());
        }
        Ok(())
    }

    fn validate_charset(charset: &str) -> Result<(), Box<dyn Error>> {
        if charsets::get_charset_from_label(charset).is_none() {
            return Err(format!("Invalid charset: {}", charset).into());
        }
        Ok(())
    }
}
