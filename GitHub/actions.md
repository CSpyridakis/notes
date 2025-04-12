# GitHub Actions

## General Notes

---

### 1. Workflow Location and Naming
GitHub Actions workflows are stored under `.github/workflows/`.

Each workflow is defined in its own YAML file:
```text
.github/
└── workflows/
    └── ci.yml
```

At the top of the file, give your workflow a name and define when it should run:
```yml
name: CI Pipeline

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  schedule:
    - cron: '0 12 * * *'
```

---

### 2. Workflow Structure: Jobs and Steps
A GitHub Actions workflow is made up of **jobs**, and each job runs in a separate environment.

```yml
jobs:
  job-name: # job-names can be anything
    runs-on: ubuntu-latest  # Runner

    steps:
        # Step 1
      - name: Checkout Code
        uses: actions/checkout@v4       # uses: Used for created actions

        # Step 2
      - name: Run a Script
        run: |                          # run: Used for commands
            echo "Hello from GitHub Actions"
```

---

### 3. Using Secrets
Secrets should be stored securely in your GitHub repository settings and accessed like this:

```yml
- name: Use Secret
  run: echo "Secret is ${{ secrets.MY_SECRET }}"
```

---

### 4. Environment Variables

**Workflow-level env:**
```yml
env:
  GLOBAL_VAR: "hello"
```

**Step-level env:**
```yml
- name: Use ENV
  env:
    LOCAL_VAR: "hi"
  run: echo "Local: $LOCAL_VAR, Global: $GLOBAL_VAR"
```

**Set dynamically during a step:**
```yml
- name: Set dynamic ENV
  run: echo "FOO=bar" >> $GITHUB_ENV
```

---

### 5. Runners and Virtual Environments

```yml
runs-on: ubuntu-latest
```

Self-hosted runners:
```yml
runs-on: [self-hosted, linux, x64]
```

---

### 6. Sharing Data Between Steps

**Set an output from a step:**
```yml
- name: Generate Value
  id: step1
  run: echo "result=42" >> $GITHUB_OUTPUT

- name: Use Output
  run: echo "Value is ${{ steps.step1.outputs.result }}"
```

---

### 7. Reusable Workflows

**Reusable workflow:**
```yml
# .github/workflows/build.yml
on:
  workflow_call:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: echo "This is a reusable workflow"
```

**Calling workflow:**
```yml
jobs:
  call-build:
    uses: org/repo/.github/workflows/build.yml@main
```

---

### 8. Timeout and Job Control

```yml
jobs:
  my-job:
    timeout-minutes: 10
    runs-on: ubuntu-latest
```

Cancel running workflows on new pushes:
```yml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

---

### 9. Job Dependencies and Execution Order

```yml
jobs:
  job1:
    runs-on: ubuntu-latest
    steps:
      - run: echo "Build"

  job2:
    needs: job1
    runs-on: ubuntu-latest
    steps:
      - run: echo "Test after build"
```

---

### 10. Caching for Faster Builds

```yml
- name: Cache Python packages
  uses: actions/cache@v4
  with:
    path: ~/.cache/pip
    key: ${{ runner.os }}-pip-${{ hashFiles('**/requirements.txt') }}
    restore-keys: |
      ${{ runner.os }}-pip-
```

---

## Boilerplate Code Snippets

### Checkout Repository on a Specific Branch
You can check out the current repository or a different one.

[Documentation](https://github.com/actions/checkout)

```yml
steps:
  - name: Checkout Repository
    uses: actions/checkout@v4
    with:
      ref: 'branch-name' # Specify the branch to check out
      repository: 'user/repository-name' # Optional: for remote repositories
      token: ${{ secrets.REPOSITORY_SECRET_KEY }} # Required for private or remote repos with write access
      fetch-depth: 0 # Full history for complete access
```
---

### Commit Changes to Repository
First checkout into the repository following [this guide](#--checkout-repository-on-specific-branch).

```yml
- name: Commit Changes
  run: |
    git config --global user.name "github-actions[bot]"
    git config --global user.email "41898282+github-actions[bot]@users.noreply.github.com"
    git add .
    git commit -m "Your commit message" || echo "No changes to commit"
    git push origin target-branch
```

**Required permissions:**
```yml
job-name:
  runs-on: ubuntu-latest
  permissions:
    contents: write
```

---

### Archive and Restore Data

#### Archive into a `.tar.gz`
```yml
- name: Archive Folder
  run: |
    tar -czvf archive.tar.gz -C folder-to-archive .
```

#### Extract Archive
```yml
- name: Extract Archive
  run: |
    tar -xzvf archive.tar.gz
    rm -f archive.tar.gz
```

---

### Upload and Download Artifacts

Official documentation is available [here](https://github.com/actions/upload-artifact).

#### Upload Artifact
```yml
- name: Upload Artifact
  uses: actions/upload-artifact@v4
  with:
    name: artifact-name
    path: path/to/target
```

#### Download Artifact
```yml
- name: Download Artifact
  uses: actions/download-artifact@v4
  with:
    name: artifact-name
    path: ./
```

---

### Use Matrix Builds

```yml
jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [14, 16, 18]
    steps:
      - uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
      - run: npm install && npm test
```

---

### Conditional Steps


GitHub Actions allows you to control whether a step should run by using the `if:` keyword with expressions.

---

#### Run only on a specific branch

```yml
- name: Run only on main
  if: github.ref == 'refs/heads/main'
  run: echo "This runs only on the main branch"
```

---

#### Run on multiple branches

```yml
- name: Run on main or develop
  if: github.ref == 'refs/heads/main' || github.ref == 'refs/heads/develop'
  run: echo "Main or Develop branch"
```

---

#### Run if specific files changed

Requires `paths` trigger in the workflow:

```yml
on:
  push:
    paths:
      - 'src/**'
```

Then in your step:

```yml
- name: Run if source files changed
  if: github.event_name == 'push'
  run: echo "Changes in src/ triggered this"
```

---

#### Run on a pull request only

```yml
- name: Run on PRs only
  if: github.event_name == 'pull_request'
  run: echo "Triggered by a pull request"
```

---

#### Run only if previous step failed

```yml
- name: This runs only if previous step failed
  if: failure()
  run: echo "Handling failure"
```

---

#### Run only if previous step succeeded

```yml
- name: Success only
  if: success()
  run: echo "Previous step succeeded"
```

---

#### Skip step on forked repositories

Useful to protect secrets or certain steps:

```yml
- name: Skip on forks
  if: github.repository_owner == 'your-org'
  run: echo "Running only on original repo"
```

---

#### Use output from a previous step

```yml
- name: Set flag
  id: flag
  run: echo "my_output=true" >> $GITHUB_OUTPUT

- name: Run if flag is true
  if: steps.flag.outputs.my_output == 'true'
  run: echo "Flag is true"
```

---

#### Run only for scheduled events

```yml
- name: Run only on schedule
  if: github.event_name == 'schedule'
  run: echo "This was triggered by a scheduled event"
```

---

#### Run if a job failed (within a matrix)

```yml
- name: Send notification on failure
  if: always() && job.status == 'failure'
  run: echo "Notify: Job failed"
```

---