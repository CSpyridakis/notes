# High availability

High availability in a nutshet:

If a Node goes down --> The VMs?CTs will automatically start on another Node.

> [!IMPORTANT]
> In order to have High availability, at **least** 3 servers are required!

**How it works?**
Health checks are sent periodically and if a server is not reachable, then they other servers handle the extra needs. 

## Enable HA for a VM 
1. Make sure that the target VM has a shared storage as a disk (read [this](./clustering.md#enable-shared-storage)).
2. Shutdown VM
3. `Datacenter` > `HA` > `Add`
   - `VM`: Select target VM