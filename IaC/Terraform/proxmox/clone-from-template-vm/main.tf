terraform {
    required_providers {
        proxmox = {
            source = "Telmate/proxmox"
            version = "3.0.1-rc6"
        }
    }
}

provider "proxmox" {
    pm_api_url          = var.pm_api_url
    pm_api_token_id     = var.pm_api_token_id
    pm_api_token_secret = var.pm_api_token_secret
    pm_tls_insecure     = true
    pm_debug            = true
}

# Example template names: 'ubuntu-desktop-template'
# This should exist on your Proxmox server as linked-clone ready template

# VM 1: Ubuntu Server
resource "proxmox_vm_qemu" "ubuntu_server" {
    name            = "ubuntu-server-vm"
    vmid            = 645                   # FIXME: Make sure that it is available

    target_node     = var.pm_target_node
    clone           = var.template_vm
    full_clone      = true

    # IMPORTANT make sure qemu agent is installed in the VM template
    # If this is set to 1 and it is NOT properly enabled on the template
    # then you will NOT be able to boot
    agent           = 1  # <--- Required for IP discovery (See below)

    cores           = 2
    memory          = 2048

    disk {
        # FIXME: these values maybe need to be updated
        size        = "40G"
        type        = "disk"
        storage     = "local-lvm"
        slot        = "scsi0"
        discard     = true
    }

    boot            = "order=scsi0;net0"
    bootdisk        = "scsi0"

    network {
        id          = 0
        model       = "virtio"
        bridge      = var.network_bridge
        firewall    = false
        link_down   = false
    }
}

output "ubuntu_server_ip" {
    value = proxmox_vm_qemu.ubuntu_server.default_ipv4_address
}
