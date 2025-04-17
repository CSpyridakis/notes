# DHCPCD - (DHCP client)

### Reassign DHCP IP address
```bash
# 0. Find the interface that you want to use 
ip link show

# 1. Kill all dhcpcd processes and release IP
sudo dhcpcd -k <interface>

# 2. Flush existing IP addresses on the interface
sudo ip addr flush dev <interface>

# 3. Start dhcpcd fresh
sudo dhcpcd <interface>
```