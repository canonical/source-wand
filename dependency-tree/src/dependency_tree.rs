use std::{ffi::OsStr, path::PathBuf};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyTree {
    pub dependencies: Vec<DependencyNode>,
}

// impl DependencyTree {
//     pub fn from_file(file: PathBuf) -> Result<DependencyTree> {
//         if let Some(extension) = file.extension() {
//             match extension.to_str() {
//                 Some("json") => {
//                     // serde_json::from
//                 },
//                 Some("yaml") | Some("yml") => {

//                 },
//             }
//         }
//         else {
//             bail!("You provided a file with no extension, allowed extensions are: .json, .yaml and .yml");
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencyNode {
    Tree(DependencyTreeNode),
    List(DependencyListNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub license: String,
    pub repository: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyTreeNode {
    pub project: Project,
    pub dependencies: Vec<DependencyTreeNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyListNode {
    pub name: String,
    pub version: String,
    pub license: String,
    pub repository: String,
}
