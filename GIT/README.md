# Git
[Distributed](./concepts.md) Version Control System.

---

```mermaid
sequenceDiagram
  participant Stash
  participant Workspace
  participant Staging
  participant Local Repository
  participant Remote Repository

  %% -------------------------
  %% Initialize
  %% -------------------------
  opt Initialize
    Local Repository ->> Workspace: git init
    Remote Repository ->> Workspace: git clone
  end

  %% -------------------------
  %% Update
  %% -------------------------
  opt Update
    Remote Repository ->> Local Repository: git fetch
    Local Repository ->> Workspace: git merge/rebase
    Remote Repository ->> Workspace: git pull
  end

  %% -------------------------
  %% Develop
  %% -------------------------
  opt Develop
    Workspace ->> Staging: git add/mv/rm
    Staging ->> Local Repository : git commit
    Workspace ->> Local Repository : git commit -a
    Local Repository ->> Remote Repository: git push
  end

  %% -------------------------
  %% Revert
  %% -------------------------
  opt Revert
    Local Repository->> Workspace: git checkout HEAD/git reset --hard
    Staging ->> Workspace: git checkout 
    Local Repository ->> Staging: git reset --soft
  end

  %% -------------------------
  %% Revert
  %% -------------------------
  opt Stash
    Workspace ->> Stash: git stash
    Stash ->> Workspace: git stash apply
  end
```

---

## Configuration
Git allows customization through its configuration system. 

| File | Command to retrieve  | Description | 
| -----| ---------------------| -----|
| `/etc/gitconfig` | `git config --system --list` | **System-wide** applies to all users on the system
| `~/.gitconfig` or `~/.config/git/config` | `git config --global --list` | **Global**: Current user
| `.git/config` | `git config --local --list` | **Local**: Specific to the current repository

- Manually edit configuration files:
  ```bash
  git config --<system/global/local> -e
  ```

- Set configurations
  ```bash
  git config --<system/global/local> <key> <value>
  ```

- Resetting Configurations
  ```bash
  git config --global --unset <key>
  ```

- Set your name and email
  ```bash
  git config --global user.name "Your Name"
  git config --global user.email "you@example.com"
  ```

- Change the default branch name 
  ```bash
    git config --global init.defaultBranch main
    ```

- Set your preferred text editor
  ```bash
  git config --global core.editor "code --wait"  # VSCode (or use 'vim/nano')
  ```

- Set a merge tool for resolving conflicts:
  ```bash
  git config --global merge.tool "vimdiff" # Or you can use 'code'
  ```

- Credential Helper
  ```bash
  git config --global credential.helper cache           # Cache credentials temporarily
  git config --global credential.helper store           # Save credentials permanently
  git config --global credential.helper 'manager-core'  # Use the Git Credential Manager
  ```

- Aliases
  ```bash
  git config --global alias.st status
  git config --global alias.co checkout
  git config --global alias.br branch
  git config --global alias.hist "log --oneline --graph --decorate --all"
  ```

- Define the default behavior for `git push`
  ```bash
  git config --global push.default simple          # Push current branch to its matching upstream
  git config --global push.default current         # Push only the current branch
  git config --global push.default matching        # Push all matching branches
  ```

- Define the default behavior for `git pull`
  ```bash
  git config --global pull.rebase true             # Set rebase as the default behavior when pulling
  ```

- Ignore changes to specific files (useful for config files with local tweaks)
  ```bash
  git update-index --assume-unchanged <file>
  ```

  - List all files marked as "assume unchanged"
  ```bash
  git ls-files -v | grep '^[a-z]'
  ```

  - Remove "assume unchanged" flag
  ```bash
  git update-index --no-assume-unchanged <file>
  ```

- Sign commits and tags with a GPG key:
  ```bash
  git config --global user.signingkey <your-gpg-key-id>
  git config --global commit.gpgSign true
  ```

- Use a specific SSH key for Git:
  ```bash
  git config --global core.sshCommand "ssh -i ~/.ssh/custom_key"
  ```

---

## Log

Simple
```bash
git log --oneline --graph --decorate --all --color --date=short --pretty=format:"%C(yellow)%h%Creset %C(green)(%ad)%Creset %C(blue)%an%Creset %C(red)%d%Creset - %s"
```

Extended
```bash
git log --oneline --graph --decorate --all --color --date=short --stat --pretty=format:"%C(yellow)%h%Creset %C(green)(%ad)%Creset %C(blue)%an%Creset %C(red)%d%Creset - %s"
```

Log Customization
```bash
git config --global log.date relative            # Show relative dates (e.g., "2 days ago")
git config --global log.showSignature true       # Show GPG signatures in log output
```

Show commit changes
```bash
git show <commit-hash>
```

---

## Debugging & Troubleshooting

| Command	| Description |
| --------| ------------|
| `git bisect` |	Identifies the commit that introduced a bug by performing a binary search between a known good and bad commit.
| `git blame -L <line-start>,<line-end> <filename>` |	Shows who last modified each line of a file, helping to identify when and why changes were made.
| `git log -S <string> --oneline` |	Searches for commits that added or removed a specific string in the code.
| `git grep -n <branch>` | 	Searches for specific text patterns in your repository.
| `git diff <commit1> <commit2>` |	Displays differences between commits, branches, or the working directory.
| `git show <commit-hash>:<file-path>` |	Shows details of a specific commit/file, including changes and metadata.
| `git reflog <branch>` |	Tracks updates to branch heads, helping to recover lost commits or branches.


---

## Pull vs Fetch

- **git fetch**: Downloads changes from the remote repository but doesn’t automatically merge them, allowing you to review changes before merging.
- **git pull**: Downloads and integrates changes (fetch + merge) from the remote repository into your local branch.

---

## Branches

### Create branch
Before
```mermaid
gitGraph
  commit id: "M1"
  commit id: "M2"
  commit id: "M3"
```

```bash
# Option 1:
git branch develop      # Create new branch
git checkout develop    # Switch to new branch

# Option 2:
git checkout -b develop

git branch # (Optional) Verify in which branch you are

```

After
```mermaid
gitGraph
  commit id: "M1"
  commit id: "M2"
  commit id: "M3"
  
  branch develop
  commit id: " M3" type: HIGHLIGHT
```

### Delete branch

* Deletes a branch if fully merged
`git branch -d <branch>`

* Force-deletes a branch
`git branch -D <branch> `

---

## Checkout vs Switch
Git `Checkout` is a versatile command but can be confusing because it serves multiple purposes, hence `Switch` was introduced in Git 2.23 as a simpler and more intuitive way to switch branches.

| Feature                          | `git checkout`                              | `git switch`                                    |
|----------------------------------|---------------------------------------------|------------------------------------------------|
| **Purpose**                      | Multi-functional: switch branches, restore files, or check out commits. | Focused on switching branches only.           |
| **Simplicity**                   | Can be confusing because of its multiple roles. | Intuitive, as it's specific to branch switching. |
| **New Branch Creation**          | `git checkout -b <branch>`                  | `git switch -c <branch>`                       |
| **Restoring Files**              | Yes (e.g., `git checkout <commit> -- <file>`). | No. Use `git restore` instead.                |
| **Detached HEAD**                | Supports checking out commits directly, leading to a detached HEAD state. | Supports detached HEAD state via `--detach`.


### Checkout
Switch branches:
`git checkout <branch-name>`

Create a new branch and switch to it:
`git checkout -b <new-branch-name>`

Restore a specific file to a previous state:
`git checkout <commit-hash> -- <file-path>`

Check out a specific commit (detached HEAD state):
`git checkout <commit-hash>`

### Switch
Switch branches:
`git switch <branch-name>`

Create a new branch and switch to it:
`git switch -c <new-branch-name>`

Switch to a specific commit (detached HEAD state):
`git switch --detach <commit-hash>`

---

## Git Merge
Combines the histories of two branches by creating a **merge commit**.

Before
```mermaid
gitGraph
  commit id: "M1"
  commit id: "M2"
  commit id: "M3"
  
  branch develop
  commit id: "D1"

  checkout main
  commit id: "M4"

  checkout develop
  commit id: "D2"

  checkout main
  commit id: "M5"

  checkout develop
  commit id: "D3"

  checkout main

```

```bash
git checkout main
git merge develop
```

After
```mermaid
gitGraph
  commit id: "M1"
  commit id: "M2"
  commit id: "M3"
  
  branch develop
  commit id: "D1"

  checkout main
  commit id: "M4"

  checkout develop
  commit id: "D2"

  checkout main
  commit id: "M5"

  checkout develop
  commit id: "D3"

  checkout main
  merge develop
```

**Key Characteristics:**
- Retains the history of both branches as-is.
- Maintains the context of branches (e.g., shows the branches diverged and then merged).
- Produces a non-linear history (with multiple parent commits).

**Use Case:**
- When you want to preserve the full history of changes.
- Collaborative workflows where you need a clear record of branch merges.

> [!TIP]
> Prefer rebasing for feature branches to avoid complex merge conflicts.

---

## Git Rebase
Moves (or replays) commits from one branch onto another branch, rewriting history.

Before
```mermaid
gitGraph
  commit id: "M1"
  commit id: "M2"
  commit id: "M3"
  
  branch develop
  commit id: "D1"

  checkout main
  commit id: "M4"

  checkout develop
  commit id: "D2"

  checkout main
  commit id: "M5"

  checkout develop
  commit id: "D3"

  checkout main
```

```bash
git checkout develop
git rebase main
```

After
```mermaid
gitGraph
  commit id: "M1"
  commit id: "M2"
  commit id: "M3"
  commit id: "M4"
  commit id: "M5"
  
  branch develop
  commit id: "D1"
  commit id: "D2"
  commit id: "D3"

  checkout main
```

**Key Characteristics:**
- Creates a linear history by applying commits sequentially.
- Results in a clean, linear history without merge commits.
- Original branch history is rewritten, so it changes commit hashes.

**Use Case:**
- When you want a clean history, such as for feature branches before merging into main.
- Preferable in personal or solo projects to avoid cluttered commit logs.

### Other commands
1. Interactive Rebase (Allow you to edit, squash, reorder, or remove commits during the rebase process).
`git rebase -i <branch>`
2. If a Rebase results in conflicts:
  - Git will pause and notify you of the conflict.
  - Resolve the conflicts manually.
  - After resolving conflicts, complete the rebase:
  `git rebase --continue`
  - If you want to cancel the rebase after a conflict:
  `git rebase --abort`

> [!TIP]
> Avoid rebasing public/shared branches to prevent rewriting history.
 
---

## Cherry-pick
Useful when you want to copy individual changes from a branch without merging it entirely.

Before
```mermaid
gitGraph
  commit id: "M1"
  commit id: "M2"
  commit id: "M3"
  
  branch develop
  commit id: "D1"

  checkout main
  commit id: "M4"

  checkout develop
  commit id: "D2"

  checkout main
  commit id: "M5"

  checkout develop
  commit id: "D3"

  checkout main
```

```bash
git checkout main
git cherry-pick D2 D1 # Commit hashes, comma separated list, the order matters
```

After
```mermaid
gitGraph
  commit id: "M1"
  commit id: "M2"
  commit id: "M3"
  
  branch develop
  commit id: "D1"

  checkout main
  commit id: "M4"

  checkout develop
  commit id: "D2"

  checkout main
  commit id: "M5"

  checkout develop
  commit id: "D3"

  checkout main
  commit id: " D2"
  commit id: " D1"
```

**Limitations**
- Avoid cherry-picking large numbers of commits, as it can complicate history and lead to conflicts.
- Be cautious with cherry-picking between branches that have diverging histories, as it might create inconsistencies.

**Key Takeaways**
- Cherry-pick is ideal for selective changes between branches.
- It does not merge branches or their histories, only the changes from specific commits.
- Always resolve conflicts carefully when cherry-picking across branches with differences.

### Other commands

1. Cherry-pick a Range of Commits
`git cherry-pick a1b2c3d..e4f5g6h`

2. If a cherry-pick results in conflicts:
  - Git will pause and notify you of the conflict.
  - Resolve the conflicts manually.
  - After resolving conflicts, complete the cherry-pick:
  `git cherry-pick --continue`
  - If you want to cancel the cherry-pick after a conflict:
  `git cherry-pick --abort`

3. Add a reference to the original commit in the new commit message.
`git cherry-pick -x <commit-hash>`

4. Appliy changes without creating a commit, leaving the changes staged.
`git cherry-pick --no-commit <commit-hash>`

5. Open an editor to modify the commit message during the cherry-pick process.
`git cherry-pick --edit <commit-hash>`

> [!TIP]
> Use sparingly to avoid cluttered history

---

## Stash
Temporarily save changes in your working directory (both tracked and untracked files) without committing them, allowing you to switch branches or work on something else without losing progress.

1. Stash Changes
`git stash`
2. Stash with a Custom Message
`git stash push -m "WIP: Implementing feature X"`
3. View Stashed Changes
`git stash list`
4. Apply Stashed Changes (optional: give specific stash)
`git stash apply # stash@{n}`
5. Apply and Drop the Stash (optional: give specific stash)
`git stash drop # stash@{n}`
6. Clear All Stashes
`git stash clear`
7. Show Details of a Stash (`-p` is for full diff)
`git stash show -p stash@{n}`
8. Create a new branch from a stash and apply the changes in one step.
`git stash branch <branch-name> stash@{n}`

---

## Revert & Reset 
Both `revert` and `reset` are used to undo changes, but they work differently and serve distinct purposes.

| **Aspect**                  | **`git revert`**                                                                 | **`git reset`**                                                                 |
|-----------------------------|----------------------------------------------------------------------------------|--------------------------------------------------------------------------------|
| **Purpose**                 | Creates a new commit that undoes the changes introduced by a specific commit.   | Moves the branch pointer to a previous commit, effectively removing subsequent commits. |
| **History**                 | **Preserves history** (does not rewrite commit history).                        | **Rewrites history** (removes commits from the history for the current branch). |
| **Use Case**                | Safe for shared/public branches (e.g., undo changes in `main`).                 | Used for private branches or local changes.                                     |
| **Effect on Commits**       | Leaves the original commit intact and creates a new commit to reverse it.       | Deletes or unstages commits based on the reset type.                           |
| **Types**                   | No variants; always creates a new commit.                                       | Has three modes: `--soft`, `--mixed`, `--hard`.                                |
| **Conflict Handling**       | May require conflict resolution if the revert affects later changes.            | Conflicts arise only with unmerged changes during `--hard` reset.              |



| Scenario	| Use git revert	| Use git reset | 
| ----------| --------------- | --------------|
| Undo a specific commit in a shared branch.	| ✅	| 🚫
| Maintain history while undoing changes.	| ✅	| 🚫
| Undo changes in a private/local branch.	| 🚫	| ✅
| Completely remove commits from history. |	🚫	| ✅

### Reset
| Mode	| Effect on Staging Area	| Effect on Working | Directory	Use Case |
| ------| ------------------------| ------------------| -------------------|
| `--soft`	| Retains changes in the staging area.	| Keeps all working directory changes.	| Undo commits but keep changes staged for editing or recommitting.
| `--mixed`	(Default)| Clears the staging area.	| Keeps all working directory changes.	| Default reset, undo commits and changes but don't delete them.
| `--hard` |	Clears the staging area.	| Discards all working directory changes.	| Permanently delete commits and changes.

**Key Takeaways**

- `git revert` is safer for shared branches, as it keeps history intact.
- `git reset` is powerful for local work but can rewrite history, so use it cautiously.
- Choose the right tool depending on whether your branch is public or private.

---

## Tags

Used to mark specific points in a repository's history. They are commonly used to identify important milestones, like releases (e.g., v1.0, v2.1.3). Tags are references to specific commits and can be either **annotated** or **lightweight**.

| Type | Description | Use case |
| -----| -------- | -----------|
| **Annotated** | Includes a message, tagger’s name, email, and a date. | Recommended for releases since it provides metadata and can be signed with GPG.
| **Lightweight** | A simple pointer to a commit, with no metadata. | Ideal for temporary or local tags.

- List all tags
  ```bash
  git tag
  ```

- List tags matching a pattern
  ```bash
  git tag -l "v1.*"
  ```

- Create Annotated Tag
  ```bash
  git tag -a <tagname> -m "Tag message"
  ```

- Create Lightweight Tag
  ```bash
  git tag <tagname>
  ```

- Tag Specific Commits
  ```bash
  git tag -a <tagname> <commit-hash> -m "Message"
  ```

- Push a specific tag to remote
  ```bash
  git push origin <tagname>
  ```

- Push all tags to remote 
  ```bash
  git push --tags
  ```

- Delete a tag locally
  ```bash
  git tag -d <tagname>
  ```

- Delete a tag on the remote:
  ```bash
  git push origin --delete <tagname>
  ```

- Renaming Tags
  ```bash
  git tag <new-tagname> <commit-hash>       # Create a new tag with the correct name
  git tag -d <old-tagname>                  # Delete the old tag locally
  git push origin <new-tagname>             # Push the new tag to the remote
  git push origin --delete <old-tagname>    # Delete the old tag on the remote
  ```

- Create a signed tag
  ```bash
  git tag -s <tagname> -m "Message"
  ```

- Verify a signed tag
  ```bash
  git tag -v <tagname>
  ```

**Best Practices**
- Use **annotated** tags for **production** releases.
- Follow a clear naming convention (e.g., vX.Y.Z for semantic versioning).
- Push tags to the remote as part of your release process.
- Use signed tags for better security and verification.

---

## Hooks
Git hooks are scripts that are automatically triggered by Git events, such as commits, merges, or pushes. These scripts allow you to enforce policies, automate tasks, and customize your Git workflow. 

**Hooks Directory**: `.git/hooks`

> [!NOTE]
> Git provides sample hook scripts with .sample extensions (e.g., pre-commit.sample). Rename or copy them to make them executable

**Types of Hooks**
| **Type**          | **Hook Name**         | **Trigger Event**                                  | **Common Use Case**                                          |
|--------------------|-----------------------|---------------------------------------------------|-------------------------------------------------------------|
| **Client-Side**    | `pre-commit`         | Before a commit is recorded                       | Linting, formatting, running tests.                        |
|                    | `prepare-commit-msg` | Before the commit message editor opens            | Modifying the default commit message template.             |
|                    | `commit-msg`         | After the commit message is entered               | Validating the commit message format.                      |
|                    | `pre-push`           | Before pushing to a remote                        | Running tests, checking branch protection policies.         |
|                    | `post-merge`         | After a successful merge                          | Installing dependencies or performing cleanup tasks.        |
|                    | `post-checkout`      | After switching branches or checking out a commit | Updating the working directory or running scripts.          |
|                    | `pre-rebase`         | Before rebasing                                    | Validating changes or preventing dangerous rebases.         |
| **Server-Side**    | `pre-receive`        | Before a push is accepted on the server           | Enforcing branch policies, running server-side validations. |
|                    | `update`             | Before updating references in a push              | Validating individual refs before applying updates.         |
|                    | `post-receive`       | After a push is accepted                          | Triggering CI/CD pipelines, notifications, or deployments.  |
|                    | `post-update`        | After references are updated                      | Updating server-side content, like generating static pages. |

- **Create hook from sample**
  ```bash
  mv .git/hooks/pre-commit.sample .git/hooks/pre-commit
  chmod +x .git/hooks/pre-commit
  ```

- **Create custom hook**: Create an executable file titled as needed (see table) in the hooks directory, example:
  ```bash
  #!/bin/bash
  echo "Running pre-commit checks..."
  # Example: Lint all staged files
  npm run lint
  if [ $? -ne 0 ]; then
      echo "Linting failed. Aborting commit."
      exit 1
  fi
  ```

- If necessary, you can bypass Git hooks using the `--no-verify` flag
  ```bash
  git commit --no-verify
  git push --no-verify
  ```

**Best Practices**

1. Keep Hooks Simple and Fast
   Time-consuming tasks in hooks can frustrate developers. Offload heavy tasks to CI/CD pipelines.

2. Make Hooks Portable
   Store hooks in a shared location and symlink them during setup.
   Example: `ln -s ../../hooks/pre-commit .git/hooks/pre-commit`

3. Use Hook Managers
  Tools like [Husky](https://typicode.github.io/husky/) or [pre-commit](https://pre-commit.com/) simplify managing hooks across teams.

1. Document Hook Behavior
  Let your team know what hooks do and how they can modify or bypass them.