# User management

## > User accounts vs System accounts

| Type |	Description	| Typical UID Range  |
| ---- | -------------- | ------------------ | 
| System Users |Used by system services (e.g., www-data, mysql)	| UID 0–999 or 1–999
| Account Users	| Human users who log in and use the system	| UID 1000+ |
    
- System users often:
    - Have no login shell (/usr/sbin/nologin)
    - Don’t own real home directories

- Regular users:
    - Can log in
    - Have passwords and home directories (e.g., /home/username)

## > User & Group files

### /etc/passwd
This file stores basic user account information. Each line represents one user, with fields separated by `:`.

```bash
username:password:UID:GID:GECOS:home_directory:shell
```

| Field | Description |
| ------ | ---------- |
| username |	Login name |
| x	| Password (stored in /etc/shadow) |
| UID |	User ID |
| GID |	Primary Group ID |
| GECOS | Full name / comment |
| home_directory | User's home directory |
| shell	 | Default shell |

### /etc/shadow (requires root access)
Each line represents one user, with 9 colon-separated fields:

```bash
username:password:lastchg:min:max:warn:inactive:expire:reserved
```
| Name       | Description                                                             |
|------------|-------------------------------------------------------------------------|
| username   | Login name (must match `/etc/passwd`)                                   |
| password   | Encrypted password or status info (see below table)                     |
| lastchg    | Days since Jan 1, 1970 that password was last changed                   |
| min        | Minimum days between password changes (0 = any time)                    |
| max        | Maximum days password is valid (forces expiry)                          |
| warn       | Days before expiration to warn the user                                 |
| inactive   | Days after expiration until account is disabled                         |
| expire     | Absolute account expiration date (days since Jan 1, 1970)               |
| reserved   | Reserved for future use (usually blank)                                 |

| password          | Meaning                                  |
|-------------------|------------------------------------------|
| \$6\$...          | Encrypted password (e.g., SHA-512)       |
| * or !            | Account is locked (cannot log in)        |
| !!                | Password not set (locked until created)  |
| "" (empty string) | No password required (not secure)        |

---

## > Common operations

### Users info

- List users info: `cat /etc/passwd`
- Show current user info: `id`
- Show different user's info: `id <username>`
- Show groups of current user: `groups`
- Show groups of another user: `groups <username>`
- Pulls data from **/etc/passwd**: `getent passwd <username>`
- Shows group info including members: `getent group <groupname>`

### Login History & Sessions

- Shows the login history of users (pulled from **/var/log/wtmp**): `last <username>`
- Displays the most recent login of all users (reads from **/var/log/lastlog**): `lastlog`
- Shows who is currently logged in (basic): `who`
- Shows who is currently logged in (includes what they’re doing, idle time, etc): `w`
- Shows who is currently logged in (just the usernames): `users`

---

### > Add user account

#### 1. useradd
```bash
sudo useradd <username>
```
\* Different distributions may behave in a different way, for this reason `/etc/default/useradd` defined the default behavior of this command.

##### Explicit Commands
- Create home dir: `sudo useradd -m <username>`
- Create system user: `sudo useradd -r <username>`
- Set login shell: `sudo useradd -s <shell/path> <username>`
 
#### 2. adduser
```bash
sudo adduser <username>
```

---

### > Passwords

#### passwd

- Set current user password: `passwd`
- Set different user: `sudo passwd <username>`
- Lock password for a specific user (This prepends a ! to the encrypted password in /etc/shadow, making it invalid.): `sudo passwd -l <user`
- Unlock user password: `sudo passwd -u  <username>`

#### chage

- Lists password expiration info: `sudo chage -l <username>`
- Set password expiration: `sudo chage -E <YYYY-MM-DD> <username>`
- Force password change on next login: `sudo chage -d 0 <username>`
- Set password expiration in X days: `sudo chage -M <X> <username>`
- Warn user X days before expiration: `sudo chage -W <X> <username>`
- Disable account after X inactive days post-expiration: `sudo chage -I <X> <username>`
- Expire the account immediately (disabling the account): `sudo chage -E 0 <username>`

---

### > Modify User 

##### Explicit Commands
- Add user to group: `sudo usermod -aG <group> <username>`
- Changes login shell: `sudo usermod -s <shell> <username>`
- Lock the account (equivalent to **passwd -l**): `sudo usermod -L <username>`
- Disable login shell: `sudo usermod -s /usr/sbin/nologin <username>`

> [!IMPORTANT]
> A. Fully Disable a User
> ```
> sudo passwd -l <username>
> sudo usermod -s /usr/sbin/nologin <username>
> sudo chage -E 0 <username>
> ```
> 
> This ensures:
>   - The password is locked.
>   - The shell is replaced with nologin (cannot log in).
>   - The account is expired.
>
>
> B. To re-enable the account:
> ```
> sudo passwd -u <username>
> sudo usermod -s /bin/bash <username>
> sudo chage -E -1 <username>
> ```
> This will:
>   - Unlock the Password (Removes the ! from the password hash in /etc/shadow)
>   - Restore a Valid Shell (like /bin/bash, /bin/zsh)
>   - Remove the Expiration Date

---

### > Delete user

#### userdel
```bash
sudo userdel <username>
```

> [!NOTE]
> userdel DOES NOTE remove user home directory by default

##### Explicit Commands
- Remove home dir too: `sudo userdel -r <username>`

--- 

###  > Groups
- Create a group: `sudo groupadd <groupname>`
- Modify group name: `sudo groupmod -n <newname> <oldname>`
- Add user to a group (append): `sudo usermod -aG <group> <username>`
- Remove user from a group: `sudo gpasswd -d <username> <groupname>`