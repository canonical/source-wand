# Structs
- OutputData (`cli/src/commands/dependencies.rs`)
    - Used in `dependencies.rs` when outputting data

## DependencyTreeNode
- DependencyTreeNode (`dependency-analysis/src/dependency_tree_node.rs`)
```
pub struct DependencyTreeNode {
    pub project: Project,
    pub dependencies: Vec<Box<DependencyTreeNode>>,
}
```

- [ ] Integrate DependencyTreeNodeGo
    - Add rdependnecies???
    - Need to replace dependnecies to a `RefCell<Vec<Rc<DependencyTreeNode>>>`




## Project
- [ ] Integrate `GoProject` with this

- Fields
    - Name (The Module Name)
    - Version
    - License
    - Repository (The Repository URL)
    - Subdirectory (for `monorepository`) >> Option<String>
    - Checkout (The version to checkout in RepoURL) >> Option<String>
- What To Add
    - sourcecraft_track
    - sourcecraft_name
    - sourcecraft_risk


