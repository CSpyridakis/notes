# Files on Unix

## 1. How a File is Stored on `ext` File Systems

```
+-----------------------+                    +--------------------------+
|                       |                    |         INODE            |
|       file.txt        |                    |   Stores Metadata        |
|                       |                    |                          |
+-----------------------+  --------------->  |  1. File type            |
|                       |                    |  2. Access rights        |
|                       |                    |  3. Number of hardlinks  |
|                       |                    |  4. File size            |
|                       |                    |  5. Last modified date   |
|                       |                    |  6. Last access date     |
|                       |                    |  7. Where is the data    |
|                       |                    |     physically stored    |
|                       |                    |  ...                     |
|                       |                    |                          |
+-----------------------+                    +--------------------------+

                                              ||
                                              ||
                                              \/

                                    +---------------------+
                                    |                     |
                                    |    Data on Disk     |
                                    |      (Usually)      |
                                    |                     |
                                    +---------------------+
```

---

## 2. How a Folder is Stored on `ext` File Systems

```
+-----------------------+                    +--------------------------+
|                       |                    |         INODE            |
|       folder          |                    |   Stores Metadata        |
|                       |                    |                          |
+-----------------------+  --------------->  |  1. Access rights        |
|                       |                    |  2. Permissions          |
|                       |                    |  3. Where is the data    |
|                       |                    |     physically stored    |
|                       |                    |  ...                     |
|                       |                    |                          |
+-----------------------+                    +--------------------------+

                                              ||
                                              ||
                                              \/
                                        ...............
```

> [!IMPORTANT]
> **Inodes DO NOT Store File/Folder Names!**

---

## Useful Commands

### View Inode Information
| Command          | Description                                  |
|------------------|----------------------------------------------|
| `ls -i`          | Lists inodes for files                      |
| `stat <file>`    | Displays inode contents for a file           |
| `df -i`          | Shows used/available inodes per filesystem   |

---

## File Types

| File Type        | Symbol | Command                                   |
|------------------|--------|-------------------------------------------|
| Ordinary File    | `-`    | `touch file`                              |
| Directory        | `d`    | `mkdir dirname`                           |
| Symbolic Link    | `l`    | `ln -s target link` *                      |
| Hard Link        | `-`    | `ln target link`   **                       |
| Character Device | `c`    |                                          |
| Block Device     | `b`    |                                          |
| Named Pipe       | `p`    |                                          |
| Socket           | `s`    |                                          |

\* **Symbolic Links**: Reference another file or directory (can be on different filesystems and resolved at runtime).

** **Hard Links**: Files point to the same inode (restricted to the same filesystem and cannot be used for directories).

### Example: Create Hard Links
```bash
cp -al source dest      # Creates a new directory structure where each file is a hard link.
```
