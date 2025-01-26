# Ports
Part of the **Transport Layer (Layer 4)** in the [OSI model](./osi.md), which includes **Transmission Control Protocol (TCP)** and **User Datagram Protocol (UDP)**.

**Port Numbers**: Each service or application typically listens on a specific port. 
These port numbers range from 0 to 65535 and are divided into three main categories:

1. **Well-known ports (0-1023)**: These are reserved for common protocols like HTTP, HTTPS, etc.
2. **Registered ports (1024-49151)**: These are used by software applications and services that are not reserved by the Internet Assigned Numbers Authority (IANA).
3. **Dynamic or private ports (49152-65535)**: These are used for ephemeral or temporary communication, often chosen dynamically by client applications for outbound connections.

## Most common ports
| No. | Port Number | Protocol/Service        | Description                                              | Encrypted |
|-----|-------------|-------------------------|----------------------------------------------------------|-----------|
| 1   | 20          | **FTP (Data)**          | File Transfer Protocol (data).                          | No        |
| 2   | 21          | **FTP (Control)**       | File Transfer Protocol (control).                       | No        |
| 3   | 22          | **SSH**                 | Secure Shell for secure remote access.                  | Yes       |
| 4   | 22          | **SFTP**                | Secure File Transfer Protocol, uses SSH.                | Yes       |
| 5   | 23          | **Telnet**              | Unencrypted remote access (deprecated).                 | No        |
| 6   | 25          | **SMTP**                | Simple Mail Transfer Protocol for sending emails.       | No        |
| 7   | 25          | **ESMTP**               | Extended SMTP for additional features.                  | No        |
| 8   | 53          | **DNS**                 | Domain Name System for resolving domain names.          | No        |
| 9   | 69          | **TFTP**                | Trivial File Transfer Protocol (simple file transfer).  | No        |
| 10  | 80          | **HTTP**                | Used for standard web traffic.                          | No        |
| 11  | 80          | **NGINX (HTTP)**      | High-performance HTTP server and reverse proxy for unencrypted traffic. | No        |
| 12  | 88          | **Kerberos**            | Authentication protocol used in various systems.        | Yes       |
| 13  | 110         | **POP3**                | Post Office Protocol for receiving emails.              | No        |
| 14  | 143         | **IMAP**                | Internet Message Access Protocol for emails.            | No        |
| 15  | 161         | **SNMP**                | Simple Network Management Protocol.                     | No        |
| 16  | 162         | **SNMP (Trap)**         | Simple Network Management Protocol (Trap).              | No        |
| 17  | 389         | **LDAP**                | Lightweight Directory Access Protocol.                  | No        |
| 18  | 443         | **HTTPS**               | Secure web traffic using SSL/TLS encryption.            | Yes       |
| 19  | 443         | **NGINX (HTTPS)**     | High-performance HTTP server and reverse proxy with SSL/TLS encryption. | Yes       |
| 20  | 445         | **SMB**                 | Server Message Block for file sharing.                  | Yes       |
| 21  | 853         | **DNS over TLS**        | DNS queries over TLS for secure DNS.                    | Yes       |
| 22  | 993         | **IMAPS**               | Secure IMAP using SSL/TLS.                              | Yes       |
| 23  | 995         | **POP3S**               | Secure POP3 using SSL/TLS.                              | Yes       |
| 24  | 2375        | **Docker Daemon API**   | Docker API for communication (unencrypted).             | No        |
| 25  | 2376        | **Docker Daemon API**   | Docker API for communication (encrypted).               | Yes       |
| 26  | 3000        | **Grafana**             | Monitoring and visualization platform.                  | No        |
| 27  | 3306        | **MySQL**               | Default port for MySQL database.                        | No        |
| 28  | 3389        | **RDP**                 | Remote Desktop Protocol for remote Windows access.      | Yes       |
| 29  | 5432        | **PostgreSQL**          | Default port for PostgreSQL database.                   | No        |
| 30  | 6379        | **Redis**             | In-memory key-value database for caching, message brokering, and real-time analytics. | No (can enable TLS) |
| 31  | 6443        | **Kubernetes API**      | Kubernetes API server for cluster communication.        | Yes       |
| 32  | 8080        | **Tomcat**              | Web server and servlet container.                       | No        |
| 33  | 8080        | **Jenkins**             | Automation server for CI/CD pipelines.                  | No        |
| 34  | 9090        | **Prometheus**          | Monitoring and alerting toolkit.                        | No        |
| 35  | 9092        | **Apache Kafka**        | Distributed event-streaming platform.                   | No        |
| 36  | 9200        | **Elasticsearch API**   | Elasticsearch RESTful API.                              | No        |
| 37  | 27017       | **MongoDB**             | Default port for MongoDB database.                      | No        |
