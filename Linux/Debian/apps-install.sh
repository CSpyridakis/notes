#!/bin/bash

# A simple script to easily install some of my go-to apps in a debian based system

sudo apt update -y

sudo apt install -y \
	git vim nemo samba pdfarranger \
	meld evince xournal gscan2pdf \
	texlive texlive-full \
	vlc filezilla speedtest-cli \
	kdeconnect indicator-multiload \
	wireshark aircrack-ng nmap zenmap \
	john macchanger steghide cmatrix \
	traceroute whois xbindkeys xautomation \
	solaar network-manager \
	btop htop gparted gdu fzf \
	software-properties-common dconf-editor expect hardinfo openvpn caffeine psensor gnome-system-monitor gnome-tweaks clamav clamtk tmux tree figlet ranger ncdu calcurse xclip neofetch iperf3 bmon iftop nload ptunnel tcpdump chromium-browser 


# Communication 
sudo snap install skype
sudo snap install telegram-desktop
sudo snap install caprine               # Facebook messenger
# Slack

#ZSH fonts
git clone https://github.com/powerline/fonts.git --depth=1  
cd fonts
./install.sh

# Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Docker 
bash <(curl -sL https://gist.githubusercontent.com/CSpyridakis/0dd4e045dcddc68496c8403c098e0c19/raw/0fda1a194f559b05d6e26311c54090abe0cba4ca/install-docker.sh)

# Lazygit
LAZYGIT_VERSION=$(curl -s "https://api.github.com/repos/jesseduffield/lazygit/releases/latest" | \grep -Po '"tag_name": *"v\K[^"]*')
curl -Lo lazygit.tar.gz "https://github.com/jesseduffield/lazygit/releases/download/v${LAZYGIT_VERSION}/lazygit_${LAZYGIT_VERSION}_Linux_x86_64.tar.gz"
tar xf lazygit.tar.gz lazygit
sudo install lazygit -D -t /usr/local/bin/

# Lazydocker
curl https://raw.githubusercontent.com/jesseduffield/lazydocker/master/scripts/install_update_linux.sh | bash
