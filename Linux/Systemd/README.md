# Systemd

`Systemd` is a **system** and **service** manager for Linux operating systems. 

It's used to **boot** the system, **manage** services, and **handle** system states. 

It has largely replaced older init systems like **SysVinit** in most modern Linux distributions.

Systemd relies on [cgroups](../cgroup/README.md) to control processes.

---

## Units

Configuration files that define system resources.

## Types of systemd Units

| Unit Type   | Extension     | Purpose                                                                 |
|-------------|---------------|-------------------------------------------------------------------------|
| [service](./service.md)   | `.service`    | Manages daemons (background services)                                   |
| [target](./target.md)    | `.target`     | Groups units together to reach a specific system state                  |
| [timer](./timer.md)     | `.timer`      | Schedules tasks like cron jobs                                          |
| `socket`    | `.socket`     | Manages sockets, often used to trigger services on-demand               |
| `mount`     | `.mount`      | Controls mounting of file systems                                       |
| `automount` | `.automount`  | Automatically mounts file systems when accessed                         |
| `device`    | `.device`     | Represents hardware devices detected by the kernel                      |
| `path`      | `.path`       | Watches paths (files/directories) and triggers services when they change|
| `snapshot`  | `.snapshot`   | Captures and restores current states of units                           |
| `swap`      | `.swap`       | Manages swap space on the system                                        |
|    ...      | ....          | ...


### Unit files Locations

| Path | Description |
| ---  | ----------- |
|  `/lib/systemd/system/`    |  Contains unit files installed by the OS vendor (distro maintainers) |
|  `/usr/lib/systemd/system/`  | Default unit files provided by packages (apt/dnf).     |
|  `/run/systemd/system/`    |   Runtime unit files, these are temporary, created at boot or dynamically.  |
|  `/etc/systemd/system/`    |   Contains user/system administrator overrides or custom unit file. Files here take precedence over **/lib** or **/usr/lib**.   |

Priority Order:
`/etc/systemd/system/` **>** `/run/systemd/system/` **>** `/lib/systemd/system/` or `/usr/lib/systemd/system/`

> [!NOTE]
> Systemd used more paths under the hood
> `systemd-analyze --system unit-paths`

### Unit files Content

Most of the unit files, follow the structure below
```
[Unit]
...

[<Unit-type>]
...

[Install] 
...
```

> [!IMPORTANT]
> Be careful,  `%` is a placeholder in `Unit files`, if you need to use it, do it like this: `%%`

#### Edit Unit files 

**Overwrite**
`systemctl edit <unit-name>.<unit-type>`

A new dir will be created `/etc/systemd/system/<unit-name>.<unit-type>.d/`, in which the override files will be stored.

**Full**
`systemctl edit --full <unit-name>.<unit-type>`

#### Create new
`systemctl edit --force --full <unit-name>.<unit-type>`

#### Cat
`systemctl cat <unit-name>.<unit-type>`
This combines all the configuration files based on the priority.

> [!IMPORTANT]
> Every time you do add a unit file, it is a good practice to run
> ```
> sudo systemctl daemon-reload
> ```

---

## Common commands

- Display General Systemd Status: `systemctl`
- Check Status of all Units: `systemctl status`
- Check Status of a Specific Unit: `systemctl status <unit-name>`
- List All Installed Unit Files:  `systemctl list-unit-files --type=<unit-type>`
- List Only Active Units of a Specific Type: `systemctl list-units --type=<unit-type>`
- Enable and Start a Service Immediately: `sudo systemctl enable --now <service-name>.service`

---
