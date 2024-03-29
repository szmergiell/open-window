mod cli_input;
mod cli_output;

use clap::Parser;
use cli_input::CliInput;
use cli_output::CliOutput;
use human_panic::setup_panic;
use owlib::open_window::measurement::Measurement;
use owlib::open_window::open_window_result;
use owlib::open_window::relative_humidity::RelativeHumidity;
use owlib::open_window::temperature::Temperature;
use std::error::Error;

fn run() -> Result<(CliOutput, bool), Box<dyn Error>> {
    let args = CliInput::parse();

    let indoor_humidity = RelativeHumidity::try_new(args.indoor_humidity)?;
    let indoor_temperature = Temperature::try_new(args.indoor_temperature)?;
    let indoor_measurement = Measurement {
        temperature: indoor_temperature,
        relative_humidity: indoor_humidity,
    };

    let outdoor_humidity = RelativeHumidity::try_new(args.outdoor_humidity)?;
    let outdoor_temperature = Temperature::try_new(args.outdoor_temperature)?;
    let outdoor_measurement = Measurement {
        temperature: outdoor_temperature,
        relative_humidity: outdoor_humidity,
    };

    let open_window_result = open_window_result(&indoor_measurement, &outdoor_measurement);

    let cli_output = CliOutput {
        indoor_dew_point: open_window_result.indoor_dew_point,
        outdoor_dew_point: open_window_result.outdoor_dew_point,
        open_window: open_window_result.open_window,
    };

    Ok((cli_output, args.json))
}

fn main() -> Result<(), Box<dyn Error>> {
    setup_panic!();

    match run() {
        Ok((output, json)) => {
            if json {
                let json = serde_json::to_string(&output)?;
                println!("{json}");
            } else {
                println!("Indoor dew point: {:.2}", output.indoor_dew_point);
                println!("Outdoor dew point: {:.2}", output.outdoor_dew_point);
                let message = match output.open_window {
                    true => "Open window!",
                    false => "Close window!",
                };
                println!("{message}");
            }
            std::process::exit(0)
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1)
        }
    }
}
