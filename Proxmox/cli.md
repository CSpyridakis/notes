# CLI

You can also manage VM/CT from the CLI. E.g. connect via SSH to a Node and handle VM/CTs.

## VM
qm (qemu manager)

|  Command  |  Description  |
| --------- | ------------- |
| `qm list` | List all VMs (this list includes also templates)
| `qm status <vmid>` | Check the current status of the VM
| `qm start <vmid>` | Start a VM
| `qm shutdown <vmid>` | Shutdown a VM
| `qm reboot <vmid>` | Reboot VM
| `qm reset <vmid>` | Hard reboot - **Use it ONLY when you have to**
| `qm stop <vmid>` | Hard poweroff - **Use it ONLY when you have to**
| `qm config <vmid>` | Get VM configuration
| `qm set --<option-name> <value> <vmid>` | Set a specific option
| `qm set --onboot 1 <vmid>` | E.g. Start VM during Node boot
| `qm create <vmid> --name <name> --memory <MB>` | Create a new VM 
| `qm destroy <vmid>` | Delete a VM - **Careful, this is irreversible** 
| `qm clone <vmid> <newid> --name <name> --full` | Clone a VM (from existing VM or template)
| `qm listsnapshot <vmid>` | List all snapshots of a VM
| `qm snapshot <vmid> <snapname>` | Take a snapshot of a VM
| `qm delsnapshot <vmid> <snapname>` | Delete a VM snapshot
| `qm rollback <vmid> <snapname>` | Rollback VM to a snapshot
| `qm monitor <vmid>` | Access the QEMU monitor shell for advanced VM control


## CT

pct (proxmox container toolkit)

|  Command     |  Description   |
| ------------ | -------------- |
| `pct list` | List all LXC containers(this list includes also templates)
| `pct status <vmid>` | Show current status of the container
| `pct start <vmid>` | Start a container
| `pct shutdown <vmid>` | Gracefully shutdown a container
| `pct reboot <vmid>` | Reboot a container
| `pct stop <vmid>` | Force stop a container - **Use it ONLY when you have to**
| `pct destroy <vmid>` | Delete a container - **Careful, this is irreversible** 
| `pct config <vmid>` | Show container configuration 
| `pct enter <vmid>` | Get a shell inside the container (like SSH, but local)
| `pct set <vmid> --<option> <value>` | Set a specific config option
| `pct exec <vmid> -- <command>` | Run a command inside the container
| `pct create <vmid> <template> --rootfs <storage>:<size>`| Create a new container from a template
| `pct clone <vmid> <newid> --name <name>` | Clone an existing container
| `pct resize <vmid> <volume> <size>` | Resize container disk (e.g. `rootfs`)
| `pct snapshot <vmid> <snapname>` | Take a snapshot of a container
| `pct delsnapshot <vmid> <snapname>` | Delete a container snapshot
| `pct rollback <vmid> <snapname>` | Rollback container to a snapshot
| `pct listsnapshot <vmid>` | List all snapshots of the container