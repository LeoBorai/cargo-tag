use anyhow::Result;

use cargo_tag::cli::Cli;
use clap::Parser;

fn main() -> Result<()> {
    let Cli::Tag(args) = Cli::parse();
    args.command.exec(&args)
}
