use std::env::current_dir;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::Result;
use semver::Version as SemVer;
use serde::Deserialize;

use crate::version::Version;

const CARGO_TOML: &str = "Cargo.toml";

/// Metadata for the `Cargo.toml` file loaded
#[derive(Debug, Default)]
pub struct Metadata {
    path: PathBuf,
}

/// A `Cargo.toml` file's representation as a struct
#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub(crate) package: Package,
    #[serde(skip_deserializing)]
    pub(crate) meta: Metadata,
}

/// Representation of the `Cargo.toml` `package` section
#[derive(Debug, Deserialize)]
pub struct Package {
    /// Crate version
    pub(crate) version: SemVer,
}

impl CargoToml {
    /// Attempts to read a `Cargo.toml` in the current directory and retrieves
    /// an instance of `CartToml` from it.
    pub fn open() -> Result<Self> {
        let mut path = current_dir()?;
        path.push(CARGO_TOML);

        let file = read_to_string(&path)?;
        let mut package: CargoToml = toml::from_str(&file)?;

        package.meta.path = path;

        Ok(package)
    }

    /// Update's current `Cargo.toml` version
    pub fn write_version(&self, version: &Version) -> Result<()> {
        let file_str = read_to_string(&self.meta.path)?;
        let mut document = file_str.parse::<toml_edit::DocumentMut>()?;

        document["package"]["version"] = toml_edit::value(version.ver.to_string());

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.meta.path)?;

        file.write_all(document.to_string().as_bytes())?;
        Ok(())
    }

    /// Executes `cargo fetch`. This is useful after updating the version in
    /// the `Cargo.toml` file to ensure `Cargo.lock` has the correct version.
    pub fn run_cargo_fetch(&self) -> Result<()> {
        let mut cmd = Command::new("cargo");

        cmd.arg("fetch");
        cmd.stderr(Stdio::inherit());
        cmd.status()?;

        Ok(())
    }
}
