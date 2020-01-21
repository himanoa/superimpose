# superimpose

Superimpose is snapshot test helper for command execution result.

## Usage

```zsh
$ mkdir snapshots
$ SUPERIMPOSE_SNAMSHOT_DIR=snapshots superimpose ls

Create snapshot!

# ↓exit_code == 0
$ SUPERIMPOSE_SNAMSHOT_DIR=snapshots superimpose ls

$ touch foobar

# ↓exit_code == 1
$ SUPERIMPOSE_SNAMSHOT_DIR=snapshots superimpose ls
```
