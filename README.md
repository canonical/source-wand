# Source Wand
Source Wand is a tool that helps you analyze and mirror source code.

# Usage
## Dependency analysis
Generate dependency tree of local directory.
```bash
source-wand dependencies local /path/to/directory
```

Generate dependency tree of git repository.
```bash
source-wand dependencies git /url/of/git/repository
```

You can format the output in json or yaml.
```bash
source-wand dependencies --format json local .
source-wand dependencies --format yaml local .
```

You can flatten the dependency tree to a list of unique dependencies.
```bash
source-wand dependencies --flatten local .
```

You can also combine the flatten and format arguments.
```bash
source-wand dependencies --flatten --format json local .
```
