# Analyzing dependencies
The first fundamental use case for `source-wand` is to analyze the dependencies of an open-source project.
At the most basic level, `source-wand` will generate a tree of the dependencies of your project.

Then, you can format and alter the analysis to:
 - Get a flat list of unique dependency/version pairs
 - Get the minimal set of dependencies required to build your project

# Generating a dependency tree
If you want to generate the dependency tree of a project, run the following command. In this case, we are assuming the project you are analyzing is https://github.com/canonical/chisel.
```bash
source-wand dependencies git https://github.com/canonical/chisel
```

This will clone the repository, detect which language/build system is used (Golang in this case) and run the right commands to generate the dependency tree. Finally, it pases that tree into a format that is common for all languages so that the output can be formatted as detailed in the next sections in the same way for all languges.

## Formatting a dependency tree
You can use the `--format [json|yaml]` on the `dependencies` command to convert the tree into either `json` or `yaml`.

For example, to get `json`:
```bash
source-wand dependencies --format=json git https://github.com/canonical/chisel
```

Or to get `yaml`:
```bash
source-wand dependencies --format=yaml git https://github.com/canonical/chisel
```

# Flattening a dependency tree
When you generate the full dependency tree, it is possible that multiple transitive dependencies depend on the same other dependency. In that case, the full tree will contain deplicates.

Furthermore, if you want to simply iterate over all dependencies using a tree, you need to use a tree traversal algorithm which can be overkill for many situations.

Using the `--flatten` flag can solve these problems.

Run the following to generate a unique list of dependency name/version pairs.
```bash
source-wand dependencies --flatten git https://github.com/canonical/chisel
```

You can combine `--flatten` with `--format`:
```bash
source-wand dependencies --flatten --format=json
```

# Generating the minimal set of dependencies required to build
Generating the full dependency tree can give you a great first impression of the dependency situation of a given project. However, sometimes not all dependencies are used in the build process.

This is the case for Golang, which selects only one version per dependency following the MVS algorithm.

Instead of using the `--flatten` flag, which will give you unique pairs of dependency name/version, use:
```bash
source-wand dependencies --minimal-build-requirements git https://github.com/canonical/chisel
```

This will return only the dependencies that are actually used in the build process.

You can also combine `--minimal-build-requirements` with `--format`:
```bash
source-wand dependencies --minimal-build-requirements --format=json git https://github.com/canonical/chisel
```

---

Next tutorial: [Replicating a project](/source-wand/tutorials/3-replicating-a-project)
