# Remote Desktop Protocol

See [https://www.digitalocean.com/community/tutorials/how-to-enable-remote-desktop-protocol-using-xrdp-on-ubuntu-22-04](https://www.digitalocean.com/community/tutorials/how-to-enable-remote-desktop-protocol-using-xrdp-on-ubuntu-22-04)
for more details.

Installation:
```
sudo apt update

# Installing a Desktop Environment on Ubuntu
sudo apt install xfce4 xfce4-goodies -y

# Installing xrdp on Ubuntu
sudo apt install xrdp -y

# Make sure that is running
sudo systemctl status xrdp
```

Make sure that the firewall allows the port that RDP uses. 

Now logout from your main screen and connect via remmina or another app.