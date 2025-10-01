# Replicating a project
Once you have analyzed the dependency tree of your project, you may want to own a deep copy of the project (including its dependencies).

Reasons you may want to do this are many, here are a few examples:
 - You want to fully control the build pipeline of your packages so that there can be no supply chain attack
 - You want to customize your packages and produce in-house versions of them
 - You want to patch CVEs on the packages you use and produce patches binaries to consume

## Creating a replication project
To replicate a project along with its dependencies, you need to define a replication manifest. That manifest can be stored anywhere. For example, you can create a git repository where you store your replicatoin manifests for different project. This way, you can share them with your team and they can run them independently.

To create a replication manifest, you can run:
```bash
source-wand replication init
```

> ⚠️ This command will soon be deprecated in favor of:
> ```bash
> source-wand init
> ```

This will create a `replication.yaml` file in your working directory.

You can also create it yourself, here is the format of this file:
```yaml
project: chisel

origin:
  git: https://github.com/canonical/chisel
  reference: refs/tags/v0.9.1

destination_template:
  git: git+ssh://<path-to-a-git-organization>/replicated-$NAME
  reference: $VERSION_MAJOR.$VERSION_MINOR.$VERSION_PATCH$VERSION_SUFFIX
```

### Origin
The origin is a pointer to the project you want to replicate, use the `git` attribute to point to the repository of the project and use the `reference` attribute to tell `source-wand` which reference (commit, tag or branch) to checkout.

### Destination template
The destination template defines where the project needs to be replicated. It is a template so you can use variables that come from the individual packages (top-level project and all dependencies). This template will be applied to all packages (top-level and dependencies).

Here are the variables that are available to you:
 - `$NAME` the name of the package
 - `$VERSION` the version of the package
 - `$VERSION_MAJOR` the major version of the package (assuming scemantic versioning) i.e. `3.4.2` -> `3`
 - `$VERSION_MINOR` the minor version of the package (assuming scemantic versioning) i.e. `3.4.2` -> `4`
 - `$VERSION_PATCH` the patch version of the package (assuming scemantic versioning) i.e. `3.4.2` -> `2`
 - `$VERSION_SUFFIX` the suffix of the version string i.e. `3.4.2-20250408` -> `-20250408`
 - `$VERSION_RETROCOMPATIBLE` the shortest expression of the version that has retrocompatibility guarantee according to scemantic versioning i.e. `3.4.2` -> `3`, but `0.5.0` -> `0.5.0`

Use these variables to define the destination template, the `git` attribute is the repository URL you want for the package and the `reference` is the branch that will be created in the repository.

### Using a specific git identity
You may want to use a specific git identity. To do that, add the following to your `replication.yaml`:

```yaml
config:
  git_identity:
    username: my_username
    email: my_username@my_company.com
```

The username and email you put here will be used as git identity when creating the repository and pushing to git.

## Doing the replication
Once you created your replication manifest, you can run the following in the directory where the manifest is located:

```bash
source-wand replication apply
```

> ⚠️ This command will soon be deprecated in favor of:
> ```bash
> source-wand apply
> ```

This will run the dependency analysis to list the dependencies that you need to build the project and it will replicate all of them along with the top-level project according to your template.
