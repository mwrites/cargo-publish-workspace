use self::cli::Command;
use anyhow::{
    bail,
    Context,
    Error,
};
use clap::Parser;
use std::{
    self,
    process::{self,},
    thread::sleep,
    time::Duration,
};

use crate::{
    order_crates::{
        order_crates_for_publishing,
        CrateInfo,
    },
    publish::publish_workspace,
    utils::print_style,
};

mod cli;
mod order_crates;
mod publish;
#[macro_use]
mod utils;

// Common Types
type Result<T = (), E = Error> = std::result::Result<T, E>;

// Constants
pub const ENV_VAR_CRATES_IO_TOKEN: &str = "CRATES_IO_TOKEN";
pub const ENV_VAR_CI_TAG: &str = "CI_TAG";

pub fn main() {
    let args = Command::parse();

    if let Err(err) = args.exec() {
        print_error!(&format!("{err:?}"));
        process::exit(1);
    }
}
