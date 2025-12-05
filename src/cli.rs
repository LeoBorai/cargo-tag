use clap::{Parser, Subcommand};

use crate::cargo_toml::CargoToml;
use crate::git::Git;
use crate::version::Version;

const ABOUT: &str = r#"Cargo plugin to bump crate's versions and Git tag them
for release.

"cargo tag" helps to automate the process of bumping versions
similar to how "npm version" does.

When bumping versions with "cargo tag", the
Cargo.toml and Cargo.lock files are updated with the new version, then a Git
commit and a Git tag are both created."#;

#[derive(Parser)]
#[command(bin_name = "cargo")]
#[command(next_line_help = true)]
#[command(name = "cargo", author, version, about, long_about = Some(ABOUT))]
pub enum Cli {
    Tag(TagArgs),
}

#[derive(clap::Args, Debug)]
pub struct TagArgs {
    #[command(subcommand)]
    pub command: Command,

    /// Prefix string for Git tags
    #[arg(short, long)]
    pub prefix: Option<String>,

    /// Get name and email from environment variables CARGO_TAG_NAME and CARGO_TAG_EMAIL.
    /// They must be set beforehand.
    #[arg(long)]
    pub env: bool,
}

#[derive(Clone, Subcommand, Debug)]
pub enum Command {
    /// Print current package version
    Current,
    /// Bumps crate's minor version and create a git tag
    Minor,
    /// Bumps crate's major version and create a git tag
    Major,
    /// Bumps crate's patch version and create a git tag
    Patch,
}

impl Command {
    pub fn exec(&self, prefix: String, env: bool) -> Result<(), Box<dyn std::error::Error>> {
        match *self {
            Command::Current => {
                let cargo_toml = CargoToml::open().unwrap();

                println!("{}", cargo_toml.package.version);
            }
            Command::Major | Command::Minor | Command::Patch => {
                let cargo_toml = CargoToml::open()?;
                let repository = if env {
                    Git::from_env("main")
                } else {
                    Git::from_git_config("main")
                };
                let mut version = Version::from(&cargo_toml.package.version);

                match self {
                    Command::Major => version.bump_major(),
                    Command::Minor => version.bump_minor(),
                    Command::Patch => version.bump_patch(),
                    _ => unreachable!(),
                };

                cargo_toml
                    .write_version(&version)
                    .expect("Failed to write version to Cargo.toml");

                cargo_toml
                    .run_cargo_fetch()
                    .expect("Failed to run `cargo fetch`");

                let version_str = prefix + version.to_string().as_str();
                repository
                    .commit(&format!("chore: bump version to {}", version_str))
                    .expect("Failed to commit files");

                repository
                    .tag(&version_str, "chore: bump version to {}")
                    .expect("Failed to create Git tag");
            }
        }

        Ok(())
    }
}
