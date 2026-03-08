# git-trans

A translation toolbox for projects using git.

## 使用指南

1. 若要开始翻译，首先需要执行 `git trans init` 初始化项目，初始化后会在项目根目录下创建一个 `.trans` 文件夹，里面会生成 `records.toml` 文件，它的目录结构会与 repo 根目录一致（已经初始化过的 repo 就不用执行这步了）。
2. 然后执行 `git trans add <file>...` 可以添加需要翻译的文件，它会将文件复制到 `.trans` 文件夹下，同时在 `records.toml` 文件中跟踪该文件对应的 git revision。
3. `git trans show` 可以查看当前项目中记录的对应状态的文件。
4. `git trans rm <file>...` 可以删除不需要翻译的文件。
5. `git trans todo`  会显示所有正在翻译（trans），正在校对（review），和未同步（unsynced）的文件。
6. 在翻译完成后，可以执行 `git trans mark review <file>...` 来标记文件为待校对，
校对完成后可以执行 `git trans mark done <file>...` 标记文件为已完成。
7. 当主 repo 更新后，需要执行 `git trans update` 来更新所有文件跟踪的 git revision。
8. 执行 `git trans diff <file>... [-g]` 可以查看文件的 diff 内容，可选的 `-g` 表示在文件所在目录下生成 git diff 格式的文件。
9. 执行 `git trans sync <file>...` 可以将文件跟踪的 git revision 同步到最新版本。
10. 执行 `git trans cover` 可以将所有翻译的文件覆盖到 repo 根目录下，以便编译生成。
11. 执行 `git trans reset` 可以将 `.trans` 文件夹以外的所有文件重置为初始状态。

## 一般的工作流

1. 执行 `git trans add <file>...` 添加需要翻译的文件。之后修改 .trans 文件夹下对应的文件。
2. 执行 `git trans cover` 覆盖所有翻译的文件到 repo 根目录下，然后 `make` 编译生成。
3. 执行 `git trans reset` 重置所有文件到初始状态。
4. 执行 `git add` 添加新翻译的文件并提交。注意保持 `.trans` 文件夹以外的内容为未修改状态。
5. 当主 repo 更新后，需要执行 `git trans update` 来更新所有文件跟踪的 git revision。
6. 如果不想更新某个文件的 git revision，可以执行 `git trans lock <file>...` 来锁定该文件，
执行 `git trans unlock <file>...` 可以解锁该文件。

## Workflow

```sh
git trans init
git trans add <file>...
git trans rm
git trans todo
git trans diff <file>... -g
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
- [x] todo 显示所有 trans，review 和 unsynced 的文件，-a 显示所有文件状态
- [x] log  显示 .trans 文件夹下所有文件的修改历史
- [ ] info <file>
- [x] diff <file>
- [x] gendiff <file>
- [x] sync <file>  同步文件到最新版本
- [x] cover
- [x] reset
- [x] show
  + [x] all
  + [x] todo
  + [x] review
  + [x] done
  + [x] synced
  + [x] unsynced
  + [x] locked
  + [x] unlocked
- [x] mark
  + [x] trans
  + [x] review
  + [x] done
- [x] lock
- [x] unlock
