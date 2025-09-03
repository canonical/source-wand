use anyhow::Result;

use crate::context::Context;

pub trait Transformation: Send + Sync + TransformationClone {
    fn apply(&self, ctx: Context) -> Result<Option<String>>;
    fn should_skip(&self, ctx: &Context) -> Option<String>;
    fn get_name(&self) -> String;
}

pub trait TransformationClone {
    fn clone_box(&self) -> Box<dyn Transformation>;
}

impl<T> TransformationClone for T where T: 'static + Transformation + Clone {
    fn clone_box(&self) -> Box<dyn Transformation> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Transformation> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
