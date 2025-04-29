# Snapshots

## Timeshift

**Common Commands**
```bash
# Show Available Snapshots
sudo timeshift --list

# Check available devices
sudo timeshift --list-devices

# Create a New Snapshot
#   --tags D: Tags the snapshot as a Demand, Weekly, Monthly, Boot
sudo timeshift --create --comments "Manual backup" --tags D

# Restore a Snapshot
sudo timeshift --restore

# Delete a Snapshot
sudo timeshift --delete --snapshot '<snapshot_name>'
```