# VLAN
VLAN stands for **Virtual LAN** and is considered as a **separate broadcast domain** or **logical networks (subnet)**.

VLAN are implemented in **Layer 2** and the only way to go from a VLAN to another is via a **Layer 3** device like a router.

### Trunking Protocols
- **802.1Q:** (Industry standard)
- **ISL:** (Cisco specific, is not prefered to be used today)
- **VTP:** 

In order to transfer VLAN information **between network devices** (e.g. switches) a trunking cable is required.

#### **802.1Q**

- **Standard Ethernet Frame (802.3)**
    | Preamble |  SFD   | Dest            | Src            | Len/Etype | Data/Payload  | FCS            |
    | -------- | ----   |---------------- | -------------- | --------- | ------------- | -------------- |
    | 7 bytes  | 1 byte | 6 bytes         | 6 bytes        | 2 bytes   | 42-1500 bytes | 4 bytes of CRC |

    Where:
        - **SFD**: Start frame delimiter
        - **Dest**: MAC destination
        - **Src**: MAC source
        - **FCS**: Frame Check Sequence
        - **CRC**: Cyclic Redundancy Check

- **802.1Q Frame**
    | Preamble |  SFD   | Dest            | Src            |  TAG        | Len/Etype | Data/Payload  | FCS            |
    | -------- | ----   |---------------- | -------------- | ----------- | --------- | ------------- | -------------- |
    | 7 bytes  | 1 byte | 6 bytes         | 6 bytes        |  (4)  bytes | 2 bytes   | 42-1500 bytes | 4 bytes of CRC |

    Where **TAG** contains:
    | TPID (0x8100) | PCP    | DEI   | VID     |
    |-------------- | ------ | ----  | ------- |
    | 2 bytes       | 3 bits | 1 bit | 12 bits |

    Where:
        - **TPID**: Tag Protocol ID - set to a value of 0x8100 for **802.1Q**
        - **PCP**: Priority code point
        - **DEI**: Drop eligible indicator (formerly CFI - Canonical Format Indicator) 
        - **VID**:  VLAN identifier. Values of **0x000 & 0xFFF are reserved**. 4.094 VLANs total

### Topologies
Hardware devices create a **Physical topology** between devices, however techologies like VLANs are used to define in a **Logical** way the network. 

#### Physical Topology
```mermaid
%%{init: {'theme':'neutral'}}%%
flowchart LR
a["Machine A"]
b["Machine B"]
c["Machine C"]
d["Machine D"]
switch[("Switch")]

a <--> |0/1| switch
b <--> |0/2| switch
switch <--> |0/3| c
switch <--> |0/4| d
```

#### Logical Topology

Example, we may want A-D and B-C to be in two different LANs, instead of using 2 separate switches, we can create VLANS to achieve the same.

Actual connection:

```mermaid
%%{init: {'theme':'neutral'}}%%
flowchart LR
a["Machine A"]
b["Machine B"]
c["Machine C"]
d["Machine D"]
switch[("Switch")]

a <--> |0/1| switch
b <--> |0/2| switch
switch <--> |0/3| c
switch <--> |0/4| d

style a  fill:#bbf,stroke:#f00,stroke-width:2px,color:#888,stroke-dasharray: 10 10
style d fill:#bbf,stroke:#f00,stroke-width:2px,color:#888,stroke-dasharray: 10 10

style b fill:#712,stroke:#00f,stroke-width:3px,color:#555,stroke-dasharray: 5 5,font-size:14pt;

style c fill:#712,stroke:#00f,stroke-width:3px,color:#555,stroke-dasharray: 5 5,font-size:14pt;
```

However based on the configuration, this is equivalent to: 

```mermaid
%%{init: {'theme':'neutral'}}%%
flowchart LR
a["Machine A"]
d["Machine D"]
switch1[("Switch")]
a <--> |0/1| switch1
switch1 <--> |0/4| d

c["Machine C"]
b["Machine B"]
switch2[("Switch")]
b <--> |0/2| switch2
switch2 <--> |0/3| c

style a  fill:#bbf,stroke:#f00,stroke-width:2px,color:#888,stroke-dasharray: 10 10
style d fill:#bbf,stroke:#f00,stroke-width:2px,color:#888,stroke-dasharray: 10 10

style b fill:#712,stroke:#00f,stroke-width:3px,color:#555,stroke-dasharray: 5 5,font-size:14pt;

style c fill:#712,stroke:#00f,stroke-width:3px,color:#555,stroke-dasharray: 5 5,font-size:14pt;
```