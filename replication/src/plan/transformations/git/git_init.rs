use anyhow::Result;
use source_wand_common::project_manipulator::project_manipulator::ProjectManipulator;

use crate::plan::{context::Context, transformation::Transformation};

#[derive(Debug, Clone)]
pub struct GitInit {
    repository_url: String,
    reference: String,
}

impl GitInit {
    pub fn new(repository_url: String, reference: String) -> Self {
        GitInit { repository_url, reference }
    }
}

impl Transformation for GitInit {
    fn apply(&self, ctx: Context) -> Result<Context> {
        ctx.sh.run_shell("git init".to_string())?;
        ctx.sh.run_shell(format!("git remote add origin {}", self.repository_url))?;
        ctx.sh.run_shell(format!("git checkout --orphan {}", self.reference))?;

        Ok(ctx)
    }

    fn should_skip(&self, ctx: &Context) -> Option<String> {
        let ls_remote: Result<String> = ctx.sh.run_shell(
            format!(
                "git ls-remote --exit-code --heads {} {}",
                self.repository_url,
                self.reference
            )
        );

        if ls_remote.is_ok() {
            Some("reference already exists on remote".to_string())
        }
        else {
            None
        }
    }
    
    fn get_name(&self) -> String {
        "initialize git repository".to_string()
    }
}
