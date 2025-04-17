# Static IP
One of the ways to set static IP is using `network-manager`. Hence, from a Debian-based machine follow these steps:

1. You need netplan support.

2. Install network manager
`sudo apt install network-manager`

4. Run `ip link show` to find the desired interface. 
  
5. Inside this file store the contents provided below
`sudo nano /etc/netplan/01-anyname.yaml`
```
network:                              
  version: 2                          
  renderer: NetworkManager            
  ethernets:                          
    <interface>:                           
      dhcp4: no                       
      addresses: [<staticip>/24]   
      routes:                         
        - to: default                 
          via: <gateway>          
      nameservers:                    
          addresses: [8.8.8.8,8.8.4.4]
```
6. Then run: 
`sudo netplan try`

7. To disable another link run:
`sudo ip link set [Interface name] down`