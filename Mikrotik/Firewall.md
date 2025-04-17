# Firewall

## Port Forwarding
### WAN --> LAN port forward
```
/ip firewall nat add chain=dstnat in-interface=<interface> protocol=tcp dst-port=<src-port> action=dst-nat to-addresses=<lan-device> to-ports=<dst-port>
/ip firewall filter add chain=forward protocol=tcp dst-port=<src-port> dst-address=<lan-device> action=accept
```