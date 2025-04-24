# Clustering

> [!NOTE]
> **Best practice:** Any networks created on one Node, needs to be created on other Nodes too.

---

## Cluster

### 1. Prepare Nodes
Make sure that each node container the networks of others.

E.g. Assume **node1** contains *vmbr0* and *vmbr1*, then those two should also be present on **node2** and **node3** which we would want to connect into a cluster with **node1**.

### 2. Create Cluster
On one of the Nodes: 
- `Datacenter` > `Cluster` > `Create Cluster` 
  - Give a `Cluster Name`
  - Set `Cluster Network` based on your needs

Under `Datacenter` > `Cluster` > `Join Information` 

You have access to your **private** cluster information. **DO NOT** share these info with anyone!

Now click on:
`Datacenter` > `Cluster` > `Join Information` > `Copy Information`

### 3. Add a Node to the Cluster

Navigate to the Proxmox Web UI of the Node you want to add into the cluster. 

Then 
`Datacenter` > `Cluster` > `Join Cluster` > Paste Cluster Info > `Join...`

---

## Remove Node from Cluster

1. Make sure that you are connected via an IP that is different to the actual Node you want to remove.
2. Migrate/Remove any VMs/CTs that you do not need in this Node. Make sure that it does not have any of them.
3. Select Node > `Replications` > Make sure that all replication jobs are done, or remove any existing.
4. Shutdown the target Node.
5. **MAKE SURE** that the target Node will not try again to reconnect in the future.
6. Select one Node and from its CLI run (these will take effect to the other Nodes in the cluster):
    1. Check nodes: `pvecm nodes` 
    2. Delete node`pvecm delnode <node-hostname>`
    3. Verification: `pvecm status`
    4. Target Node configuration should still exist: `ls -l /etc/pve/nodes/<node-hostname>`
    5. So, you can delete it: `rm -rf /etc/pve/nodes/<node-hostname>`
    6. Remove from `/etc/pve/priv/known_hosts` the target Node
7. From every single operational server in the cluster run: 
   1. `systemctl stop pve-ha-crm.service`
8. Select one Node again and run:
   1. `rm -f /etc/pve/ha/manager_status`
   2. `systemctl start pve-ha-crm.service` **Afterwards, run the some from the other Nodes too.**

---

## Migrate
Select VT/CT to migrate and right click on it > `Migrate`

Then: 
- `Target node`: The Node to which you want to migrate the resource
- `Target storage`: The storage to be used (propably local-lvm)


