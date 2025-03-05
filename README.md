# git-trans

A translation toolbox for projects using git.

## git commands

```sh
# show the absolute path of the top-level directory of the working tree.
git rev-parse --show-toplevel

# find the last revision hash of files
git log -n 1 --pretty=format:%H -- <file>

# find the last revision hash and datetime of files
git log -n 1 --pretty=format:"%H%x09%ai" -- <file>

# diff changes for a file in different revisions
git diff <old-hash> HEAD <file>

# diff changes for two files in different revisions
git diff <revision_1>:<file_1> <revision_2>:<file_2>
```
