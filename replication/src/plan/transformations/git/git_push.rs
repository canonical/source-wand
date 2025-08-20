use anyhow::Result;
use source_wand_common::project_manipulator::project_manipulator::ProjectManipulator;

use crate::plan::{context::Context, transformation::Transformation};

#[derive(Debug, Clone)]
pub struct GitPush {
    repository_url: String,
    reference: String,
}

impl GitPush {
    pub fn new(repository_url: String, reference: String) -> Self {
        GitPush { repository_url, reference }
    }
}

impl Transformation for GitPush {
    fn apply(&self, ctx: Context) -> Result<Context> {
        ctx.sh.run_shell("git add .".to_string())?;
        ctx.sh.run_shell("git commit -m 'Replicate source code'".to_string())?;
        ctx.sh.run_shell(format!("git push -u origin {}", self.reference))?;

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
        "push to git".to_string()
    }
}
