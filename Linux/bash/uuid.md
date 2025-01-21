# Universally Unique Identifier (UUID)
A 128-bit or 32 hexadecimal digits 3232323number used to uniquely identify objects or entities in computer systems. 

* Standard Format: 
`xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx`

    M: The version of the UUID (e.g., version 1, 4).
    N: Indicates variant and type (e.g., standard or reserved).

    | **Version** | **Name**          | **Generation Mechanism**                                    | **Use Case**                                          |
    |-------------|-------------------|------------------------------------------------------------|------------------------------------------------------|
    | 1           | Timestamp-based   | Combines current timestamp with the MAC address of the machine. | Useful when time-ordering is required.              |
    | 2           | DCE Security      | Similar to version 1 but includes POSIX UIDs (User IDs) and other fields. | Less commonly used, mainly for legacy systems.      |
    | 3           | Name-based (MD5)  | Uses an MD5 hash of a namespace and a name. Deterministic. | Useful for generating consistent IDs from inputs.    |
    | 4           | Random            | Generated entirely randomly.                              | Most common version due to simplicity and randomness.|
    | 5           | Name-based (SHA-1)| Uses a SHA-1 hash of a namespace and a name. Deterministic. | Similar to version 3 but more secure.               |


* Generate in Linux:
  Random-Based UUIDs : `uuidgen -r`
  Time-Based UUIDs: `uuidgen -t`
  Hash-Based UUIDs (MD5): `uuidgen -m -N <domain> -n @url`
  Hash-Based UUIDs (SHA1): `uuidgen -s -N <domain> -n @url`
  Using kernel: `cat /proc/sys/kernel/random/uuid`