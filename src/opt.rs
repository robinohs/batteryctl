use clap::Parser;

use crate::{
    build::{self},
    info::command::InfoArgs,
};

pub const VERSION: &str = shadow_rs::formatcp!(
    "{} ({}@{}, {}, {})",
    build::PKG_VERSION,
    build::SHORT_COMMIT,
    build::BRANCH,
    build::RUST_VERSION,
    build::BUILD_TIME
);

#[derive(Debug, Parser)]
#[command(version = VERSION)]
#[command(name = build::PROJECT_NAME)]
#[command(about = "A CLI for battery control and battery information.", long_about= None)]
pub struct Opt {
    #[clap(short, long, help = "Enable debug output")]
    pub debug: bool,
    #[clap(subcommand)]
    pub command: Option<Subcommand>,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    // Define your subcommands here
    #[clap(about = "Display battery information [default command]")]
    Info(InfoArgs),
}
