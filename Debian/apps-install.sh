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

# VSCode


# Winbox
