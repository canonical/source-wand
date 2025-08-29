use anyhow::Result;

use source_wand_concurrent_executor::{
    context::Context,
    transformation::Transformation,
};

use crate::plan::transformations::{
    git::git_init::GitInit,
    golang::fetch_source::GolangFetchSource,
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
    fn apply(&self, ctx: Context) -> Result<Option<String>> {
        if self.git_init.reference_exists(&ctx) {
            self.git_init.apply(ctx.clone())?;
            Ok(Some("fetched back from mirror".to_string()))
        }
        else {
            self.fetch_source.apply(ctx.clone())?;
            self.git_init.apply(ctx.clone())?;
            Ok(Some("fetched from Go proxy".to_string()))
        }
    }

    fn should_skip(&self, _: &Context) -> Option<String> {
        None
    }

    fn get_name(&self) -> String {
        "initialize project".to_string()
    }
}
