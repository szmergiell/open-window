use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliInput {
    #[arg()]
    /// Indoor temperature in Celcius degress
    pub indoor_temperature: f64,
    #[arg()]
    /// Indoor relative humidity expressed as percentage (not fraction of 1)
    pub indoor_humidity: u8,

    #[arg()]
    /// Outdoor temperature in Celcius degress
    pub outdoor_temperature: f64,
    #[arg()]
    /// Outdoor relative humidity expressed as percentage (not fraction of 1)
    pub outdoor_humidity: u8,

    #[arg(short, long)]
    /// Output JSON instead of human readable messages
    pub json: bool,
}
