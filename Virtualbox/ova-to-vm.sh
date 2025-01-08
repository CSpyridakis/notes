#!/bin/bash

# Run the script using this command:
# bash <(curl -sL )

# ==================================================================
# VM setup
# ==================================================================
echo "[LOG] Create a new VM from an OVA file."                                           
read -p " - Enter the name of your VM [default: UbuntuServer]: " vm_name           
read -p " - Enter the SSH port to connect [default: 2222]: " ssh_port              
read -p " - Enter the path of the OVA file [default: ~/ubuntu-server22.04.ova]: " ova_path     
                                                                                   
vm_name=${vm_name:-"UbuntuServer"}                                                 
ssh_port=${ssh_port:-2222}                                                         
ova_path=${ova_path:-"$HOME/ubuntu-server22.04.ova"}                                               
                                                                                   
echo ""                                                                            
echo "[LOG] Create VM [${vm_name}] from [${ova_path}] and start SSH in [${ssh_port}]..." 

# ==================================================================
# Download OVA file if not already exists and its name is 
# ==================================================================
# If the ${ova_path} is the predefined and does not exist, download it
[[ ${ova_path} == "$HOME/ubuntu-server22.04.ova" ]] && [[ ! -e "${ova_path}" ]] && \
    curl -o ~/ubuntu-server22.04.ova -L https://drive.usercontent.google.com/download?id=1i7v6XzRpfIddGP4JvEsfFDDNHNhevjEO&export=download&authuser=0&confirm=t&uuid=b19b31a3-cc85-4bb2-a1a7-acab649bebf6&at=AENtkXYgPEw0grI5UCqFnIFKuls0%3A1732469974397                                                                    

# ==================================================================
# Create a VM from the OVA file
# ==================================================================

# 1. Insert OVA file to VBoxManage
VBoxManage import ${ova_path} --vsys 0 --vmname ${vm_name} --options importtovdi

# 2. Set CPU and RAM 
VBoxManage modifyvm ${vm_name} --cpus 4 --memory 4096

# 3. Enable SSH in NIC 1 (NAT network) good in cases you have to manually setup NIC 2 network
VBoxManage modifyvm ${vm_name} --natpf1="guestssh,tcp,,${ssh_port},,22"

# 4. Enable NIC 2 for WAN access (Bridged network)
br_adapt=`ip link show | grep "state UP" | head -n 1 | tr -d ' ' | cut -d ':' -f 2`
VBoxManage modifyvm ${vm_name} --nic2 bridged --bridgeadapter2 ${br_adapt}

# 5. (Optional) Make sure that the configuration is correct
echo "[LOG] VM created!"
echo "[LOG] username: ubuntu, password: ubuntu"
# VBoxManage showvminfo ${vm_name}

# 6. Start VM in Headless mode
echo "[LOG] Start VM in Headless mode"
VBoxHeadless --startvm "${vm_name}" &
sleep 4

# ==================================================================
# Congratulations, you have now a running VM, you can connect to it
# ==================================================================

# To view running VMs run
echo "[LOG] List running VMs"
VBoxManage list runningvms

# Then connect to your VM, username::password ==> ubuntu::ubuntu 
echo "[LOG] Connect to the VM"
ssh ubuntu@127.0.0.1 -p ${ssh_port}


# =======================================================================================
# Other useful commands
# ==================================================================

# To Stop the VM run
# VBoxManage controlvm ${vm_name} poweroff

# To List HDDs
# VBoxManage list hdds

# ==================================================================
# At the end, to remove your VM
# ==================================================================
# VBoxManage unregistervm ${vm_name} --delete