use anyhow::Result;
use source_wand_common::project_manipulator::project_manipulator::ProjectManipulator;

use crate::plan::{context::Context, transformation::Transformation};

#[derive(Debug, Clone)]
pub struct GitPush {
    reference: String,
    commit_text: String,
}

impl GitPush {
    pub fn new(
        reference: String,
        commit_text: String,
    ) -> Self {
        GitPush { reference, commit_text }
    }
}

impl Transformation for GitPush {
    fn apply(&self, ctx: Context) -> Result<Option<String>> {
        ctx.sh.run_shell("git add .".to_string())?;
        ctx.sh.run_shell(format!("git commit -m '{}'", self.commit_text))?;
        ctx.sh.run_shell(format!("git push -u origin {}", self.reference))?;

        Ok(Some(format!("commit \"{}\"", self.commit_text)))
    }

    fn should_skip(&self, ctx: &Context) -> Option<String> {
        if ctx.sh.run_shell("git rev-parse --is-inside-work-tree".to_string()).is_err() {
            return Some("local is not a git repository".to_string());
        }

        let clean_tree: Result<String> = ctx.sh.run_shell(
            "git diff --quiet && git diff --cached --quiet && [ -z \"$(git ls-files --others --exclude-standard)\" ]".to_string()
        );

        if clean_tree.is_ok() {
            Some("there is nothing to push".to_string())
        }
        else {
            None
        }
    }

    fn get_name(&self) -> String {
        "push to git".to_string()
    }
}
