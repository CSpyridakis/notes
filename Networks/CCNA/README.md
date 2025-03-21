# CCNA notes

## Comparison of Ethernet Technologies
| Name | Speed | Topology | Max Distance (m) | Cable Type | Connector | Max Active Devices | Technology Name | Notes  |
| ---- | ----- | -------- | ---------------- | ---------- | --------- | ------------------ | --------------- | ------ |
| 10BASE5 | 10 Mbps | Bus | 500 | Thick coaxial | Vampire Tap | ~100 nodes | Thicknet | Early Ethernet; installation was challenging due to cable rigidity.
| 10BASE2 | 10 Mbps | Bus | 185 | Thin coaxial | BNC/T-adapter | ~30 nodes | Thinnet | More flexible and cost-effective; limited segment length.
| 10BASE-T | 10 Mbps | Star | 100 per segment | Twisted-pair (UTP) | RJ45 | 1 device per port; overall network depends on switch capacity (up to ~1024 nodes)* | Ethernet over twisted pair | Uses hubs or switches; better fault isolation compared to bus topology.
| 100BASE-TX | 100 Mbps | Star | 100 per segment | Twisted-pair (UTP) | RJ45 | 1 device per port; overall network design-dependent | Fast Ethernet | Became the standard for LANs; offers higher bandwidth than 10BASE-T.
| 1000BASE-T | 1 Gbps | Star | 100 per segment | Twisted-pair (UTP, Cat5e or higher) | RJ45 | Limited primarily by switch capacity | Gigabit Ethernet | Widely deployed in modern networks; high performance over copper cabling.

## Tranmission Type
| Transmission Type | Description                                                      | Recipients                                     | Addressing Method                                          | Examples                                       |
| ----------------- | ---------------------------------------------------------------- | ---------------------------------------------- | ---------------------------------------------------------- | ---------------------------------------------- |
| Unicast           | One-to-one communication where data is sent from one sender to one specific receiver.  | Single recipient                               | Unique destination address (e.g., specific MAC or IP address).  | Web browsing, email, file transfers.            |
| Broadcast         | One-to-all communication where data is sent from one sender to all devices within a network segment.  | All nodes on the network segment               | Special broadcast address (e.g., 255.255.255.255 in IPv4).  | ARP requests, network announcements.            |
| Multicast         | One-to-many communication where data is sent to a selected group of devices that have subscribed to the multicast group.  | A specific group of recipients                 | Group address (e.g., IP multicast addresses in the range 224.0.0.0/4). | Streaming media, conferencing, real-time data feeds. |

## Devices
| Device  | OSI Layer           | Function                                                       | Intelligence                                                      | Collision Domain                                 | Bandwidth Use                                                                                   | Typical Use Case                                                           |
|---------|---------------------|----------------------------------------------------------------|-------------------------------------------------------------------|--------------------------------------------------|-------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------|
| **Hub** | Physical (Layer 1)  | Repeats incoming signals to all ports                          | None – broadcasts to every port                                   | Single collision domain for all ports            | Shared among all ports                                                                            | Small, less complex networks                                               |
| **Bridge** | Data Link (Layer 2) | Filters traffic between two LAN segments                       | Basic – learns and filters MAC addresses                          | Divides network into 2 collision domains         | Better segmentation reduces unnecessary traffic                                                 | Connecting two network segments                                             |
| **Switch** | Data Link (Layer 2) | Forwards data based on MAC addresses to specific ports           | Advanced – learns, filters, and creates separate collision domains for each port | Each port is its own collision domain            | Efficient use – dedicated bandwidth per connection (full-duplex possible)                        | Modern LAN environments requiring high performance and efficiency           |
