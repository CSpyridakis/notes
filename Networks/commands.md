## traceroute 
A diagnostic tool used to trace the path that packets take from your computer to a destination.

Usage: 
`$ traceroute <domain/ip>`

Output:
`Hop     Router     1st RTT      2nd RTT     3rd RTT`

Installation on Debian-Based systems
`sudo apt install traceroute`

Port used: `33434`
In case you want to test a server make sure that firewall rules allow connection to this port.

---

## ss
A network troubleshooting tool (socket statistics) that provides information about network connections, routing tables, interface statistics, and more.

Usage: 
`$ ss -tulnp` 

-t: Show TCP sockets.
-u: Show UDP sockets.
-l: Show listening sockets only.
-n: Display numerical addresses and port numbers.
-p: Add a column showing the PID and process name associated with each listening port

To display only from a specific port info.
`ss -tulnp 'sport' = :22` 


Deprecated alternative: `netstat`

---

# nc
A versatile networking utility (netcat) used for debugging, testing, and exploring network connections.

Listen on a port: 
`nc -lvp <port>`

Connect to a port: 
`nc -v <ip> <port>`

Check if port open: 
`nc -zv <ip> <port>`

Installation 
`sudo apt install netcat-traditional`

---

# nmap 
A tool used for network discovery and security auditing.

**CAUTION: Use this command only after authorization on the device/network you want to scan!!!**

Scan network for devices:
`nmap -sn <ip>/24`

Scan device:
`nmap <ip>`

Installation on Debian-Based systems
`sudo apt install nmap`


---

# dig
A tool used to query DNS (Domain Name System) servers to retrieve information about domains, IP addresses, and DNS records.

Query for the A Record
`dig <domain>`

Query for All Available Records
`dig <domain> ANY`

Query on a Specific DNS Server
`dig @8.8.8.8 <domain>`

Reverse DNS Lookup
`dig -x <ip>`

Deprecated alternative: `nslookup`


---

# route
The route command is used to display or modify the routing table in a computer's network stack. 

Usage:
`route -n`

Add a New Route
`sudo route add -net <destination_network> netmask <netmask> gw <gateway> <interface>`

Delete a Route
`sudo route del -net <destination_network> netmask <netmask> gw <gateway> <interface>`

Alternative: `ip route` (preferred replacement)

---

# arp
The arp (Address Resolution Protocol) command is used to view and manipulate the ARP cache in a system. 

Display the ARP Cache:
`arp -a`

Add a Static ARP Entry
`sudo arp -s <IP_address> <MAC_address>`

Delete an ARP Entry
`sudo arp -d <IP_address>`

Clear the ARP Cache
`sudo ip -s -s neigh flush all`

Installation on Debian-Based systems
`sudo apt install net-tools`

---

# mtr
The mtr (My Traceroute) command is a network diagnostic tool that combines the functionality of ping and traceroute in one program. 

Usage:
`mtr -c 10 <domain/ip>`

---

# ip 
A comprehensive tool for managing networking, routing, and addressing.

View all network interfaces
`ip a`

View routing table
`ip route`

Bring up or down a network interface:
`sudo ip link set <interface> <up/down>`

---

# tcpdump
A powerful tool for packet analysis and network traffic capture. 

`sudo tcpdump -i <interface> -p <port> -w <capture-file>.pcap`

Installation on Debian-Based systems
`sudo apt install tcpdump`

---

# nftables
Used for firewall management in Linux. 

List current rules
`sudo nft list ruleset`

Flush all rules (reset the firewall)
`sudo nft flush ruleset`

Add Rule Command
`sudo nft add rule <table> <chain> <rule>`

    For example, to allow incoming HTTP traffic (port 80) in the input chain of the filter table:
    
    `sudo nft add rule ip filter INPUT tcp dport 80 accept`

Remove Rule Command
`sudo nft delete rule <table> <chain> handle <rule_handle>`

    To delete the rule for allowing HTTP traffic from the previous example, first, you need to find the rule's handle (ID). List the rules to identify the handle:
    
    `sudo nft -a list chain ip filter INPUT`

    Then, delete the rule using its handle. For example, if the handle is N:
    
    `sudo nft delete rule ip filter INPUT handle N`

Deprecated alternative: `iptables`

Installation on Debian-Based systems
`sudo apt install nftables`

---

# curl
A command-line tool used for transferring data from or to a server, supporting a wide range of protocols (HTTP, FTP, etc.).

Fetch a webpage
`curl <domain/ip>`

Fetch a webpage and show the headers:
`curl -I <domain/ip>`

Download a file:
`curl -O <domain/ip>/file.zip`

Send a POST request with data:
`curl -X POST -d "name=value" <domain/ip>`

Installation on Debian-Based systems
`sudo apt install curl`

---

# wget
A command-line tool for downloading files from the web using HTTP, HTTPS, and FTP protocols.

Download a file and save it with a custom filename
`wget -O <file_name>.zip <domain/ip>/file.zip`

Download a file in the background:
`wget -b <domain/ip>/file.zip`

Installation on Debian-Based systems
`sudo apt install wget`