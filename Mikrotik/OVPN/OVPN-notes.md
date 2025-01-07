# OVPN notes

> **Important:**
    In order to work make sure for the following:
    
    1. The Time is corrent!
    
    2. If you are using the default configuration, that the 'defconf: drop all not coming from LAN' firewall rule is altered or disabled
    or in general that the firewall accepts connection to the OVPN server.

## 1. Create Certificates
**[System]:** &#8594;  Certificates &#8594; Certificates (Tab) 
1. Generate `CA Certificate`
    1. (+ ) New 
        1. `General` (Tab)
            1. Set `Name` to 'CA'
            2. Set `Common Name` to 'CA'
            3. Set `Key Type` to 'RSA'
            4. Set `Key Size` to '2048'
            5. Set `Days Valid` to '3650'
        2. `Key Usage` (Tab) 
            1. Check `crl sign`
            2. Check `key cert. sign`
        3. (Apply)
        4. (Sign)
            1. Set `Certificate` to 'CA'
            2. Set `CA CRL HOST` to 'xxxxx.sn.mynetname.net' # **TODO** update
            3. Click on `Start` 
            4. Wait... until "Progress" is done
            5. Close window
        5. Click on `OK`
        6. Double click on 'CA' &#8594;  General (Tab) &#8594;  Enable "Trusted" &#8594;  (OK)

 2. Generate `Server Certificate`
    1. (+ ) New 
        1. `General` (Tab)
            1. Set `Name` to 'Server'
            2. Set `Common Name` to 'Server'
            3. Set `Key Type` to 'RSA'
            4. Set `Key Size` to '2048'
            5. Set `Days Valid` to '3650'
        2. `Key Usage` (Tab) 
            1. Check `digital signature`
            2. Check `key encipherment`
            3. Check `tls server`
        3. (Apply)
        4. (Sign)
            1. Set `Certificate` to 'Server'
            2. Set `CA` to 'CA'
            3. Click on `Start` 
            4. Wait... until "Progress" is done
            5. Close window
        5. Click on `OK`
        6. Double click on 'Server' &#8594;  General (Tab) &#8594;  Enable "Trusted" &#8594;  (OK)

2. Generate `Client Certificate`
    1. (+ ) New
       1. `General` (Tab)
            1.  Set `Name` to 'Client'
            2.  Set `Common Name` to 'Client'
            3.  Set `Key Type` to 'RSA'
            4.  Set `Key Size` to '2048'
            5.  Set `Days Valid` to '3650'
        2. Key Usage (Tab) 
            1. Check `tls client`
        3. (Apply)
        4. (Sign)
            1. Set `Certificate` to 'Client'
            2. Set `CA` to 'CA'
            3. Click on `Start`
            4. Wait... until "Progress" is done
            5. Close window
        5. Click on `OK`
        6. Double click on 'Client' &#8594;  General (Tab) &#8594;  Enable "Trusted" &#8594;  (OK)

Hence, the final table should look like this: 

|      |  Name  | Common Name | Key Size | Days Valid | Trusted | CA |
| ---- | ------ | ------      | ----     | ----       | ------- | -- |
| KLAT | CA     |   CA        | 2048     | 3650       | yes     |    |
| KIT  | Client | Client      | 2048     | 3650       | yes     | CA |
| KIT  | Server | Server      | 2048     | 3650       | yes     | CA |

---

## 2. Export Certificates
**[System]** &#8594;  Certificates &#8594; Certificates (Tab)
 1. CA Certificate
     1. (Right click on 'CA') > `Export` 
         1. Set `Certificate:` to 'CA'
         2. Set `Type:` to 'PEM'
         3. Set `File Name:` to 'CA'
         4. Click on `Export`
         5. Close window
 2. Client Certificate
     1. (Right click on 'CA') > `Export`
         1. Set `Certificate:` to 'Client'
         2. Set `Type:` to 'PEM'
         3. Set `Export Passphrase:` to somethign strong
         4. Set `File Name:` to 'ovpn-client'
         4. Click on `Export`

**[Files]**
You will see 3 files. Two `.crt` files and one `.key` file
Right click on them and press `Download`

```bash
> /file print
```

--- 

## 3. OpenVPN Server Configuration in MikroTik Router
**[PPP]**
Interface (Tab) &#8594;  (OVPN Server)

1. Check `Enabled`
2. Set `Port` to '443'
3. Set `Mode` to 'ip'
4. Set `Netmask` to '24'
5. Set `Max MTU` to '1500'
6. Set `Keepalive Timeout` to '60'
7. Set `Default Profile` to 'default'
8. Set `Certificate` to 'Server'
9. Check `Require Client Certificate`
10. Uncheck all, except:
    1.  Auth    -> `sha256`
    2.  Cipher -> `aes 256 cbc`
11. (Apply) > (OK)

---

## 4. Creating OpenVPN Users
1. IP Pool Configuration
    **[IP]** &#8594;  Pool &#8594;  Pool (Tab) > (+) New
        1. Set `Name` to 'vpn_pool'
        2. Set `Addresses` to '192.168.10.101-192.168.10.149' # TODO update
        3. Set `Next Pool` to 'none'
        4. (Apply) &#8594;  (OK)

2. User Profile Configuration
    **[PPP]** &#8594;  Profiles (Tab) &#8594;  (+) New
        1. Set `Name` to 'vpn_profile'
        2. Set `Local Addresses` to '192.168.10.1' # Use Gateway address
        3. Set `Remote Address` to 'vpn_pool'
        4. (Apply) &#8594;  (OK)

3. User Configuration (Create multiple users if you want from here)
    **[PPP]** &#8594; Secrets (Tab) &#8594;  (+) New
        1. Set `Name` to your desired username
        2. Set `Password` to your desired password
        3. Set `Service` to 'ovpn'
        4. Set `Profile` to 'vpn_profile'
        5. (Apply) &#8594; (OK)
---

## 5. Now you can connect to the server
1. Use `.ovpn` file (see template and update it based on `.srt` and `.key` files

---

Code all together:

```
# ==================================================
# Generate Certificates

# Generate CA Certificate
/certificate add name=CA common-name=CA days-valid=3650 key-usage=key-cert-sign,crl-sign 
/certificate sign CA ca-crl-host=TODO.sn.mynetname.net
/certificate set CA trusted=yes

# Generate Server Certificate
/certificate add name=Server common-name=Server days-valid=3650 key-usage=digital-signature,key-encipherment,tls-server
/certificate sign Server ca=CA
/certificate set Server trusted=yes

# Generate Client Certificate
/certificate add name=Client common-name=Client days-valid=3650 trusted=yes key-usage=tls-client 
/certificate sign Client ca=CA
/certificate set Client trusted=yes

# ==================================================

# Export Certificates 
/certificate export-certificate CA file-name=CA
/certificate export-certificate Client file-name=ovpn-client export-passphrase=TODO

# ==================================================
# Enable OVPN Server
/interface ovpn-server server set enabled=yes port=443 mode=ip netmask=24 max-mtu=1500 keepalive-timeout=60 default-profile=default certificate=Server require-client-certificate=yes auth=sha256 cipher=aes256-cbc protocol=tcp 

# ==================================================
# Extra Configurations

# IP Pool Configuration
/ip pool add name=vpn_pool range=192.168.10.101-192.168.10.149

# VPN Profile Configuration
/ppp profile add name=vpn_profile local-address=192.168.10.1 remote-address=vpn_pool

# Create user for VPN
/ppp secret add name=TODO password=TODO service=ovpn profile=vpn_profile

# Firewall
/ip firewall nat add chain=srcnat src-address=192.168.10.0/24 action=masquerade
/ip firewall filter add chain=input protocol=tcp dst-port=443 action=accept
```
