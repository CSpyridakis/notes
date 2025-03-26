# pfsense

pfsense is a fully open source firewall/router based on Free-BSD.

## How it works?
pfsense is a stateful inspection firewall

Stateful inspection means that pfSense doesn't just look at each individual packet in isolation—it keeps track of the state of active connections. In other words, it maintains a table that records information about ongoing sessions, such as the source, destination, and protocol details. This allows pfSense to determine whether an incoming packet is part of an established connection or if it's unexpected (and potentially malicious).
This means that you only need to specify the request policy and the return will be automatically be applied.

The states are stored in a state table in RAM.


--- 

## Installation
Download and install from [here](https://www.pfsense.org/download/) or from [here](https://atxfiles.netgate.com/mirror/downloads/).

### 1. Partitioning

| Feature                | ZFS                                                                                     | UFS                                                                                       |
|------------------------|-----------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------|
|                        | Zettabyte File System.                                                                  | Unix File System. 
|                        | **Prefered**                                                                            | **Not prefered** 
| **Architecture**       | Pooled storage with an integrated volume manager                                        | Traditional file system operating on fixed disk partitions                                |
| **Data Integrity**     | End-to-end checksumming and self-healing capabilities                                   | Limited data integrity features; relies on lower-level RAID or manual checksumming          |
| **Snapshots & Clones** | Built-in, efficient snapshots and cloning support                                       | Some variants (like UFS2 on FreeBSD) offer snapshot support but less integrated overall     |
| **Scalability**        | Designed for large-scale storage environments and high scalability                        | Suited for simpler, smaller storage systems                                                 |
| **RAID Integration**   | Integrated RAID support (e.g., RAID-Z)                                                    | Does not include integrated RAID; external RAID solutions are typically used                |
| **Performance**        | Advanced features may add overhead; performance can vary with workload and configuration   | Generally simpler and may offer lower overhead in less complex environments                 |
| **Additional Features**| Supports built-in compression, deduplication, and dynamic striping                         | Fewer built-in features; additional functionality typically requires separate solutions      |
| **Licensing**          | Open source under the CDDL license                                                      | Often used under BSD licenses (in FreeBSD) or proprietary in some other implementations       |

### 2. Continue Installation
### 3. Boot Device
If you need to modify the default IP, do it from the pfsense console.

Set **DNS servers**

### 4. Final Setup:
1. System > Advanced > Miscellaneous
    
    **Cryptographic & Thermal Hardware**
        - Set `Cryptographic Hardware` to `AES-NI CPU-based Acceleration`
        - Set `Thermal Sensor` 

---

## 📙 Defaults
### A. Credentials
Username: `admin`
Password: `pfsense`

## Useful Packages
System -> Package Manager -> Package Installer

1. openvpn-client-export
2. snort
3. WireGuard

---

## Single NIC

If you want to install pfSense on a x86 machine with a single NIC, you can still use it, as long as you also use a managed switch with VLANs support. Then, you will be able to use one VLAN as WAN and another as LAN, connect a single Ethernet cable to the x86 machine and still work.

### Procedure
1. Install pfSense on the desired device
2. Reboot device
3. (pfSense) Setup VLANs during first boot
   1. Select as parent interface the available NIC and create a new VLAN
   2. Provide a tag for this VLAN (e.g. 10)
   3. Continue with the configuration
   4. Give for WAN the {interface}.VID 
   5. Give for LAN the {interface}
4. (switch) Setup pfsense port as **trunk** port that has the VLAN ID as specified above. Then for WAN setup this port as **access** port with the above VLAN. 

---

## Access from WAN the Webconfigurator
### Option 1: Can access pfsense via consol
Disabling packet filter (Useful for first time setup)
1. Connect to the pfsense console.
2. Choose option 8 (Shell)
3. Type `pfctl -d` This will disable the packet filtering.
4. To re-enable, execute `pfctl -e`

### Option 2: Can access through LAN the Webconfigurator
Using the web Webconfigurator
1. Firewall > rules > wan
2. Add new rule
   - `Action`: **PASS**
   - `Interface`: **WAN**
   - `Address Family`: **IPv4**
   - `Protocol`: **TCP**
   - `Source`: **Any**
   - `Destination`: **This firewall (self)**
    Place this rule at he top of the list. 

---

## Create new VLAN interface
1. `Interfaces` > `Assignments` > `VLANs` > `Add`
2. `Interfaces` > `Assignments` > `Interface Assignments` > Select it on `Available network ports` and click on `Add`
3. `Interfaces` > Click on the new interface name
   1. `Enable interface`: [x]
   2. `IPv4 Configuration Type`: Static IPv4
   3. `IPv6 Configuration Type`: Can also be `None`
   4. `IPv4 Address`: Desired gateway IP / 24
   5. `IPv4 Upstream gateway`: None
4. `Services` > `DHCP Server` > New interface
   1. `Enable DHCP server on DMZ interface`: [x]
   2. `Address Pool Range`: Set these fields too
   3. `DNS Servers`
