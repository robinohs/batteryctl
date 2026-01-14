use crate::{
    command::Command,
    features::Features,
    info::command::{InfoArgs, InfoCommand},
};
use clap::Parser;
use eyre::Result;
use log::Level;
use shadow_rs::shadow;

mod command;
mod features;
mod opt;
mod info {
    pub(crate) mod command;
    mod display_data;
}
mod battery_data;

shadow!(build);

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = opt::Opt::parse();

    // read and set log level
    let log_level = if opt.debug { Level::Debug } else { Level::Info };
    stderrlog::new()
        .verbosity(log_level)
        .timestamp(stderrlog::Timestamp::Second)
        .show_module_names(true)
        .init()?;

    let features = Features::detect()?;

    match opt.command {
        Some(opt::Subcommand::Info(args)) => InfoCommand::from(args).execute(features)?,
        None => InfoCommand::from(InfoArgs::default()).execute(features)?,
    };

    Ok(())
}
