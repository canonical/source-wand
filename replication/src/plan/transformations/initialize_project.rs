use anyhow::Result;

use crate::{
    plan::{
        context::Context,
        transformation::Transformation,
        transformations::{
            git::git_init::GitInit,
            golang::fetch_source::GolangFetchSource
        }
    }
};

#[derive(Debug, Clone)]
pub struct InitializeProject {
    git_init: GitInit,
    fetch_source: GolangFetchSource,
}

impl InitializeProject {
    pub fn new(
        git_init: GitInit,
        fetch_source: GolangFetchSource,
    ) -> Self {
        InitializeProject { git_init, fetch_source }
    }
}

impl Transformation for InitializeProject {
    fn apply(&self, ctx: Context) -> Result<Context> {
        if self.git_init.reference_exists(&ctx) {
            self.git_init.apply(ctx.clone())?;
        }
        else {
            self.fetch_source.apply(ctx.clone())?;
            self.git_init.apply(ctx.clone())?;
        }

        Ok(ctx)
    }

    fn should_skip(&self, _: &Context) -> Option<String> {
        None
    }

    fn get_name(&self) -> String {
        "initialize project".to_string()
    }
}
