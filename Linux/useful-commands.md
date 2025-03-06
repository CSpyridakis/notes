# Useful Linux commands

## Random file of size N
Create a file, containing random printable characters of specific size

`FILE_SIZE=$((1024 * 10)) ; tr -cd '[:print:]' < /dev/random | head -c ${FILE_SIZE} > file.txt`

Where:
* tr: Translates/deletes characters.
* -c: Complements the set, i.e., matches everything except the specified characters.
* -d: Deletes characters not in the set.
* [:print:]: Matches all printable characters, including spaces.

--- 

## `rsync`

### Copy files 
`rsync -ahP  <source_dir>/ <destination_dir>/`

Where: 
* -a: Preserve timestamp (among other options like recursive copy for directories)
* -h: Human Readable
* -P: Combines 
    * --progress: Display progress during transfer.
    * --partial: Useful for large files, even if the transfer is disrupted, it can be restored.

### Sync folders
`rsync -ahP --delete <source_dir>/ <destination_dir>/`

Where: 
* --delete: Will keep both folders identical, so if something exists in the destination that is not present in the source, it will be deleted.

### Move files from one dir to another
`rsync -ahP --remove-source-files <source_dir>/ <destination_dir>/`

This though will keep empty directories, it will not delete them.
A workarround is to run afterwards: `find ${source_dir} -type d -empty -delete`

### Remote transfer 

#### Local -> Server
`rsync -ahPz <local_source_dir> <user>@<ip/hostname>:/<remote_path>`

rsync uses ssh in the background.

Where: 
* -z: Compress before transfer

#### Server -> Local
`rsync -ahPz -e 'ssh -p 22' <user>@<ip/hostname>:/<remote_path> <local_destination_path>`

Where: 
* -z: Compress before transfer
* -e: Put ssh arguments on '' similar to a ssh command.