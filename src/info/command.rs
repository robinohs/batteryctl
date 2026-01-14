use clap::ValueEnum;
use eyre::Result;

use crate::{battery_data::BatteryData, features::Features, info::display_data::DisplayData};

#[derive(clap::Args, Debug, Default, Clone)]
pub struct InfoArgs {
    #[clap(short, long, value_enum, default_value_t = Display::Default, help = "Set output display format")]
    pub display: Display,
    #[clap(short, long, default_value_t = false, help = "Display temperature in Fahrenheit")]
    pub fahrenheit: bool,
    #[clap(
        short,
        long,
        default_value_t = false,
        help = "Display all potential metrics, even if unavailable on this system"
    )]
    pub all: bool,
}

#[derive(Debug)]
pub struct InfoCommand {
    args: InfoArgs,
}

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum Display {
    #[default]
    Default,
    Bare,
    Json,
}

impl From<InfoArgs> for InfoCommand {
    fn from(value: InfoArgs) -> Self {
        InfoCommand { args: value }
    }
}

impl crate::command::Command for InfoCommand {
    fn execute(&mut self, features: Features) -> Result<()> {
        let battery_data = BatteryData::gather(features)?;
        let display_data = DisplayData::generate_display_data(&battery_data, &self.args);
        let repr = match self.args.display {
            Display::Bare => display_data.into_bare_presentation(&self.args),
            Display::Default => display_data.into_default_representation(&self.args),
            Display::Json => display_data.into_json_representation(&self.args)?,
        };
        println!("{repr}");
        Ok(())
    }
}
