use mml_cli_main::impl_cli_main;

mod args;
mod error;

use crate::args::Args;
use crate::error::Error;

impl_cli_main!();

pub fn run(args: &Args) -> Result<(), Error> {
    todo!()
}
