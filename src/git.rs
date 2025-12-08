use std::env::current_dir;

use anyhow::{Context, Result};
use git2::{Config, IndexAddOption, Repository, Signature, Tree};

/// Performs Git related operations in the crate's repository
pub struct Git {
    email: String,
    name: String,
    branch: String,
    repo: Repository,
}

impl Git {
    /// Creates `Git` client from environment variables
    ///
    /// # Panics
    ///
    /// If `CARGO_TAG_EMAIL` or `CARGO_TAG_NAME` is not set
    pub fn from_env(branch: &str) -> Result<Self> {
        let email = std::env::var("CARGO_TAG_EMAIL").context("CARGO_TAG_EMAIL not set")?;
        let name = std::env::var("CARGO_TAG_NAME").context("CARGO_TAG_NAME not set")?;

        Git::open(branch, &email, &name)
    }

    /// Creates `Git` client from git config
    ///
    /// # Panics
    ///
    /// If `user.email` or `user.name` are not found
    pub fn from_git_config(branch: &str) -> Result<Self> {
        let cfg = Config::open_default().context("Cannot open git config")?;
        let email = cfg
            .get_entry("user.email")
            .context("user.email not found")?;
        let email = email.value().context("user.email not utf8")?;
        let name = cfg.get_entry("user.name").context("user.name not found")?;
        let name = name.value().context("user.name not utf8")?;

        Git::open(branch, email, name).context("Failed to open Git repository")
    }

    /// Opens the Git repository in the current working directory and uses the
    /// provided `email`, `name` and `branch` to perform Git operations like
    /// `commit` and `tag`.
    pub fn open(branch: &str, email: &str, name: &str) -> Result<Self> {
        let cwd = current_dir()?;
        let repo = Repository::open(cwd)?;

        Ok(Self {
            email: email.into(),
            name: name.into(),
            branch: branch.into(),
            repo,
        })
    }

    /// Creates a commit with instance's Email, Name and Branch with the
    /// taggging tree set. This means, adding `Cargo.toml` and `Cargo.lock`.
    pub fn commit(&self, message: &str) -> Result<()> {
        let signature = self.signature()?;
        let head = self.repo.head()?.peel_to_commit()?;
        let tree = self.tagging_tree()?;

        self.repo.commit(
            Some(&format!("refs/heads/{}", &self.branch)),
            &signature,
            &signature,
            message,
            &tree,
            &[&head],
        )?;

        Ok(())
    }

    /// Creates a Git Tag with the provided `Version`
    pub fn tag(&self, version_str: &str, message: &str) -> Result<()> {
        let tagger = self.signature()?;
        let head = self.repo.head()?.peel_to_commit()?;
        let obj = head.as_object();

        self.repo.tag(version_str, obj, &tagger, message, false)?;

        Ok(())
    }

    /// Creates a `Signature` using the instance's `email` and `name` along with
    /// the current time
    fn signature(&self) -> Result<Signature<'_>> {
        let signature = Signature::now(&self.name, &self.email)?;

        Ok(signature)
    }

    /// Creates a Git tree by adding all the files in the current repository
    fn tagging_tree(&self) -> Result<Tree<'_>> {
        let mut index = self.repo.index()?;

        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;

        index.write()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        Ok(tree)
    }
}
