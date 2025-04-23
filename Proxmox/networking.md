# Proxmox - Networking

## Create bridge to bind VMs
1. Node > `System` > `Network`
2. `Create` > `Linux Bridge`
3. Give name
4. Add comment
5. Do not give anything else if not needed
6. `Create`
7. `Apply Configuration`
8. Now you can add this bridge to a VM

## Bonds
These can be used for either failovers or link aggregation

### Failover
1. `Create: Linux Bond`
2. Give a name
3. `Slaves`: give as comma separated the interfaces
4. `Mode`: `active-backup`
5. `bond-primary`: give the primary interface name
6. Assign the bond to the desired bridge
7. `Apply Configuration`

To verify: Node > `Shell` and run `cat /proc/net/bonding/<bond-name>`