# CGroup

A **cgroup** (short for control group) in Linux, is a kernel feature that organizes and limits system resources (CPU, memory, disk I/O, etc) for a group of processes.

**Containers**, **systemd**, and other technologies utilize this feature in order to work. 

View cgroups: `systemctl status`
Inspect cgroups: `systemd-cgtop`

\*By default only 3 levels are displayed (to increase this value: `--depth=<num>`)

You can usually find cgroups mounted in: `/sys/fs/cgroup`

Slice file 
```
[Slice]
MemoryHigh=500M
```

`systemd-rum --user --slice=<slice-file>.slice <binary-path-to-attach>`