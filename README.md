# git-trans

A translation toolbox for projects using git.

## Workflow

```sh
git trans init
git trans add <file>...
git trans ls -a|-r
git trans rm
git trans status
git trans diff <file>...
git trans gendiff <file>...
git trans sync <file>...
git trans cover
# build
git trans reset
git trans log
git commit (precommit script)
git push
```

## git commands

```sh
# show the absolute path of the top-level directory of the working tree.
git rev-parse --show-toplevel

# find the last revision hash of files
git log -n 1 --pretty=format:%H -- <file>

# find the last revision hash and datetime of files
git log -n 1 --pretty=format:"%H%x09%ai" -- <file>

# diff changes for a file in different revisions
git diff <old-revision> HEAD <file>

# diff changes for two files in different revisions
git diff <revision_1>:<file_1> <revision_2>:<file_2>

# reset all files butexcept .trans folder
git restore --source=HEAD --staged --worktree . ":(exclude).trans/"

# show log only in .trans folder
git log -- .trans/
```

## Todo list

- [x] init
- [x] add <file>
- [x] rm <file>
- [ ] ls  列出当前文件夹下所有记录的文件，-r 递归
- [ ] status  显示所有 todo，review 和 unsynced 的文件，-a 显示所有文件状态
- [x] log  显示 .trans 文件夹下所有文件的修改历史
- [ ] info <file>
- [x] diff <file>
- [x] gendiff <file>
- [x] sync <file>  同步文件到最新版本
- [x] cover
- [x] reset
- [x] show
  + [ ] all
  + [x] todo
  + [x] review
  + [x] done
  + [x] synced
  + [x] unsynced
  + [x] locked
  + [x] unlocked
- [x] mark
  + [x] todo
  + [x] review
  + [x] done
- [x] lock
- [x] unlock
