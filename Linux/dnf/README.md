# DNF package manager

`dnf` (Dandified YUM) is a package manager for RPM-based Linux distributions like:
- Fedora
- RHEL (Red Hat Enterprise Linux)
- CentOS
- Rocky Linux, AlmaLinux, etc.

What it does:
- Installs, removes, updates, and manages software packages.
- Resolves dependencies automatically.
- Works with **.rpm** packages (Red Hat Package Manager format).
- Replaces the older yum tool.

Backend tool: **rpm**.

---

## Common Commands
| Command                                      | Description                                                                                           |
|----------------------------------------------|-------------------------------------------------------------------------------------------------------|
| `sudo dnf versionlock <package>`             | Do not update/upgrade a package (lock its version). Requires `sudo dnf install 'dnf-command(versionlock)'`. Useful for locking a specific kernel version. |
| `dnf search <package-name>`                  | Search for a package                                                                                  |
| `dnf info <package-name>`                    | Get detailed info about a package                                                                     |
| `sudo dnf install <package-name>`            | Install a package                                                                                     |
| `sudo dnf remove <package-name>`             | Remove a package                                                                                      |
| `sudo dnf update`                            | Update all packages                                                                                   |
| `sudo dnf update <package-name>`             | Update a specific package                                                                             |
| `dnf list installed`                         | List all installed packages                                                                           |
| `dnf list available`                         | List available packages                                                                               |
| `sudo dnf clean all`                         | Clean cache and metadata                                                                              |

