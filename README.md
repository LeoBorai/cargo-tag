<div>
  <h1 align="center">cargo tag</h1>
  <h4 align="center">
    Cargo plugin to bump crate's versions and Git tag them for release
  </h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/cargo-tag.svg)](https://crates.io/crates/cargo-tag)
  [![Documentation](https://docs.rs/cargo-tag/badge.svg)](https://docs.rs/cargo-tag)
  ![Build](https://github.com/whizzes/cargo-tag/workflows/build/badge.svg)
  ![Clippy](https://github.com/whizzes/cargo-tag/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/whizzes/cargo-tag/workflows/fmt/badge.svg)

</div>

Cargo plugin to bump crate's versions and Git tag them
for release.

"cargo tag" helps to automate the process of bumping versions
similar to how "npm version" does.

When bumping versions with "cargo tag", the
Cargo.toml and Cargo.lock files are updated with the new version, then a Git
commit and a Git tag are both created.

```
Bump crate's version and create a Git tag

Usage: cargo tag [OPTIONS] <COMMAND>

Commands:
  current
          Print current package version
  minor
          Bumps crate's minor version and create a git tag
  major
          Bumps crate's major version and create a git tag
  patch
          Bumps crate's patch version and create a git tag
  prerelease
          Sets the provided prerelease string and create a git tag
  help
          Print this message or the help of the given subcommand(s)

Options:
  -p, --prefix <PREFIX>
          Prefix string for Git tags
      --no-commit
          Skip creating a Git commit
      --no-tag
          Skip creating a Git tag
      --env
          Get name and email from environment variables CARGO_TAG_NAME and CARGO_TAG_EMAIL. They must be set beforehand
  -h, --help
          Print help
```

## Installation

```bash
cargo install cargo-tag
```

> Requires Git to be installed in your system.

Using:
```bash
cargo tag patch
# or
cargo tag -p=v patch
```

## Contributing

Every contribution to this project is welcome. Feel free to open a pull request,
an issue or just by starting this project.

## License

Distributed under the terms of both the MIT license
