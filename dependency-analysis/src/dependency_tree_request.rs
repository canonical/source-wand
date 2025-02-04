use std::path::PathBuf;

pub enum DependencyTreeRequest {
    LocalProject { path: PathBuf },
    GitProject { url: String, branch: Option<String> },
    NameBased { name: String, version: String },
}

impl DependencyTreeRequest {
    pub fn from_local_project(path: PathBuf) -> DependencyTreeRequest {
        DependencyTreeRequest::LocalProject { path }
    }

    pub fn from_git_project(url: String, branch: Option<String>) -> DependencyTreeRequest {
        DependencyTreeRequest::GitProject { url, branch }
    }

    pub fn from_name(name: String, version: String) -> DependencyTreeRequest {
        DependencyTreeRequest::NameBased { name, version }
    }
}
