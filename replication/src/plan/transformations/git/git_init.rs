use anyhow::Result;

use source_wand_common::project_manipulator::project_manipulator::ProjectManipulator;

use source_wand_concurrent_executor::{
    context::Context,
    transformation::Transformation
};

use crate::model::replication_config::GitIdentity;

#[derive(Debug, Clone)]
pub struct GitInit {
    repository_url: String,
    reference: String,
    git_identity: Option<GitIdentity>,
}

impl GitInit {
    pub fn new(
        repository_url: String,
        reference: String,
        git_identity: Option<GitIdentity>,
    ) -> Self {
        GitInit { repository_url, reference, git_identity }
    }

    pub fn reference_exists(&self, ctx: &Context) -> bool {
        let ls_remote: Result<String> = ctx.sh.run_shell(
            format!(
                "git ls-remote --exit-code --heads {} {}",
                self.repository_url,
                self.reference
            )
        );

        ls_remote.is_ok()
    }
}

impl Transformation for GitInit {
    fn apply(&self, ctx: Context) -> Result<Option<String>> {
        if self.reference_exists(&ctx) {
            ctx.sh.run_shell(format!("git clone {} .", self.repository_url))?;
            ctx.sh.run_shell(format!("git checkout {}", self.reference))?;
            ctx.sh.run_shell("git pull".to_string())?;
        }
        else {
            ctx.sh.run_shell("git init".to_string())?;
            ctx.sh.run_shell(format!("git remote add origin {}", self.repository_url))?;
            ctx.sh.run_shell(format!("git checkout --orphan {}", self.reference))?;
        }

        if let Some(git_identity) = &self.git_identity {
            ctx.sh.run_shell(format!("git config --local user.name {}", git_identity.username))?;
            ctx.sh.run_shell(format!("git config --local user.email {}", git_identity.email))?;
        }

        Ok(None)
    }

    fn should_skip(&self, _: &Context) -> Option<String> {
        None
    }

    fn get_name(&self) -> String {
        "initialize git repository".to_string()
    }
}
