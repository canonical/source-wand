use anyhow::Result;

use source_wand_common::project_manipulator::project_manipulator::ProjectManipulator;

use source_wand_concurrent_executor::{
    context::Context,
    transformation::Transformation
};

#[derive(Debug, Clone)]
pub struct GolangFetchSource {
    pub origin: String,
}

impl GolangFetchSource {
    pub fn new(origin: String) -> Self {
        GolangFetchSource { origin }
    }
}

impl Transformation for GolangFetchSource {
    fn apply(&self, ctx: Context) -> Result<Option<String>> {
        ctx.sh.run_shell(format!("cp -r {}/* .", self.origin))?;
        Ok(None)
    }

    fn should_skip(&self, _: &Context) -> Option<String> {
        None
    }
    
    fn get_name(&self) -> String {
        "fetch go source code".to_string()
    }
}
