use std::path::PathBuf;

use crate::atoms::Outcome;

use super::super::Atom;
use gitsync::GitSync;
use tracing::instrument;

#[derive(Default)]
pub struct Clone {
    pub repository: String,
    pub directory: PathBuf,
    pub reference: Option<String>,
}

impl std::fmt::Display for Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GitClone {}#{} to {:?}",
            self.repository,
            self.reference
                .clone()
                .unwrap_or_else(|| String::from("main")),
            self.directory,
        )
    }
}

impl Atom for Clone {
    #[instrument(name = "git.clone.plan", level = "info", skip(self))]
    fn plan(&self) -> anyhow::Result<Outcome> {
        Ok(Outcome {
            side_effects: vec![],
            should_run: !self.directory.exists(),
        })
    }

    #[instrument(name = "git.clone.execute", level = "info", skip(self))]
    fn execute(&mut self) -> anyhow::Result<()> {
        let git_sync = GitSync {
            repo: self.repository.clone(),
            branch: self.reference.clone(),
            dir: self.directory.clone(),
            ..Default::default()
        };

        // we may add .sync as another atom
        git_sync
            .bootstrap()
            .map_err(|err| anyhow::anyhow!("{:?}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_can_plan() {
        let temp_dir = match tempfile::tempdir() {
            std::result::Result::Ok(dir) => dir,
            std::result::Result::Err(_) => {
                assert_eq!(false, true);
                return;
            }
        };

        let git_clone = Clone {
            repository: String::from("https://github.com/comtrya/comtrya"),
            directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        assert_eq!(false, git_clone.plan().unwrap().should_run);

        let git_clone = Clone {
            repository: String::from("https://github.com/comtrya/comtrya"),
            directory: temp_dir.path().join("nonexistent"),
            ..Default::default()
        };

        assert_eq!(true, git_clone.plan().unwrap().should_run);
    }

    #[test]
    fn it_can_execute() {
        let temp_dir = match tempfile::tempdir() {
            std::result::Result::Ok(dir) => dir,
            std::result::Result::Err(_) => {
                assert_eq!(false, true);
                return;
            }
        };

        let mut git_clone = Clone {
            repository: String::from("https://github.com/comtrya/comtrya"),
            directory: temp_dir.path().join("clone"),
            ..Default::default()
        };

        match git_clone.execute() {
            Ok(_) => (),
            Err(_) => assert_eq!(false, true),
        }
    }
}
