use std::fmt;

use anyhow::{Context, Result};
use semver::Version as SemVer;

pub struct Version {
    pub(crate) ver: SemVer,
}

enum Digit {
    Major,
    Minor,
    Patch,
}

impl Version {
    pub fn bump_major(&mut self) {
        self.bump(Digit::Major)
    }

    pub fn bump_minor(&mut self) {
        self.bump(Digit::Minor)
    }

    pub fn bump_patch(&mut self) {
        self.bump(Digit::Patch)
    }

    fn bump(&mut self, digit: Digit) {
        match digit {
            Digit::Major => {
                self.ver.major += 1;
                self.ver.minor = 0;
                self.ver.patch = 0;
            }
            Digit::Minor => {
                self.ver.minor += 1;
                self.ver.patch = 0;
            }
            Digit::Patch => {
                self.ver.patch += 1;
            }
        }
    }

    pub fn set_prerelease(&mut self, prerelease: &str) -> Result<()> {
        self.ver.pre = semver::Prerelease::new(prerelease)
            .context(format!("Failed to set prerelase value \"{prerelease}\""))?;
        Ok(())
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ver)
    }
}

impl From<Version> for SemVer {
    fn from(ver: Version) -> Self {
        ver.ver
    }
}

impl From<&SemVer> for Version {
    fn from(ver: &SemVer) -> Self {
        Version {
            ver: ver.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use semver::Version as SemVer;

    use super::Version;

    #[test]
    fn bumps_major_version() {
        let current = SemVer::new(1, 1, 1);
        let mut version = Version::from(&current);

        version.bump_major();

        assert_eq!(version.ver.major, 2);
        assert_eq!(version.ver.minor, 0);
        assert_eq!(version.ver.patch, 0);
    }

    #[test]
    fn bumps_minor_version() {
        let current = SemVer::new(1, 1, 1);
        let mut version = Version::from(&current);

        version.bump_minor();

        assert_eq!(version.ver.major, 1);
        assert_eq!(version.ver.minor, 2);
        assert_eq!(version.ver.patch, 0);
    }

    #[test]
    fn bumps_patch_version() {
        let current = SemVer::new(1, 1, 1);
        let mut version = Version::from(&current);

        version.bump_patch();

        assert_eq!(version.ver.major, 1);
        assert_eq!(version.ver.minor, 1);
        assert_eq!(version.ver.patch, 2);
    }
}
