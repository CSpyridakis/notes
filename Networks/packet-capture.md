# Packet Capture

## Tools
### Capture packets via:
1. Wireshark
2. tcpdump (Linux/Mac)
3. netsh (Windows)
4. tshark (Linux/Mac)

### Analysis tools
1. Teleseer

## Wireshark
Remember to enable [x] `Enable promiscuous mode on all interfaces`

## Methods to capture packets
### 1. Use software from a device connected to the network

### 2. Use a SPAN port
A SPAN port is a managed switch port that mirrors all the traffic from all other interfaces.

Characteristics:
1. Capture packets from all devices connected to the switch
2. We need to have physical access
3. We need a managed switch
4. For high throuput/bandwidth we need a switch that can handle processing and will not drop packets

### 3. Using a network TAP (Test Access Point)
A network device that creates a copy of every packet transmitted. It is connected between the router and the switch

```mermaid
graph LR

router --> TAP --> Switch --> Devices
```