# [EBS](./ebs.md) vs [EFS](./efs.md) vs [EC2 Instance Store](./ec2.md#5-ec2-instance-store)

| Feature                 | EFS (Elastic File System)                     | EBS (Elastic Block Store)                 | Instance Store                               |
|-------------------------|-----------------------------------------------|-------------------------------------------|---------------------------------------------|
| **Type**                | Network File System                          | Block Storage                             | Ephemeral Local Storage                     |
| **Access**              | Shared by multiple EC2 instances concurrently | Dedicated to a single EC2 instance        | Attached to a single EC2 instance           |
| **Persistence**         | Persistent                                   | Persistent                                | Non-persistent; data lost on instance stop  |
| **Use Case**            | Shared file system, scalable workloads       | Persistent storage for one instance       | Temporary storage, high-speed applications  |
| **Performance**         | Scalable (General Purpose or Max I/O modes)  | High performance (gp3, io1, io2 for IOPS) | Very high IOPS, low latency                 |
| **Data Durability**     | Stored redundantly across AZs                | Stored redundantly within one AZ          | No durability; tied to the lifecycle of the instance |
| **Scalability**         | Automatic scaling (GB to PB)                 | Pre-provisioned; scale manually           | Fixed to the instance type                  |
| **Protocol**            | NFS (Network File System)                    | Block-level storage                       | Block-level storage                         |
| **Availability**        | Multi-AZ                                     | Single AZ (can use snapshots for backups) | Tied to instance                            |
| **Pricing Model**       | Pay-per-use (GB/month, IA tier for cost-saving) | Pay for provisioned capacity (GB/month)  | Included in the instance cost               |
| **Best For**            | Shared storage, web servers, machine learning | Databases, transaction-heavy apps         | Caching, temporary storage, NoSQL databases |
| **Examples**            | Media workflows, shared dev environments     | Running relational DBs, data warehousing  | High-speed caching, buffer for big data     |
| **Encryption**          | Supports encryption at rest and in transit (via KMS) | Supports encryption at rest (via KMS)      | No default encryption; application-level security |
| **Backup**              | Integrated with AWS Backup for automated backups | Snapshots provide incremental backups, stored in S3 | No backup support; ephemeral data storage |
| **Shared Access**       | Multiple instances can access simultaneously  | Single instance access, multi-attach for io2 | Single instance only                        |
| **Latency**             | Slightly higher latency compared to EBS due to network access | Low latency due to direct attachment       | Very low latency, directly attached         |
| **Limitations for AMIs (OS Type)**| - EFS supports most Linux-based OS and can be mounted on EC2 instances running a variety of Linux distributions.<br>- Not directly supported for Windows AMIs without third-party software for NFS. | - EBS supports all Linux distributions, Windows, and custom OS AMIs.<br>- Compatible with most OS types.<br>- Can be used as a root volume for EC2 instances. | - Instance store is typically used with Linux-based AMIs.<br>- Cannot be used with Windows AMIs for persistence.<br>- Data is lost when the EC2 instance is stopped or terminated, so not suitable for Windows server or long-term OS storage. |
| **Other Important Info** | - Ideal for shared file systems where data needs to be accessible from multiple instances.<br>- EFS pricing can be more complex due to different storage classes (Standard vs. IA). | - EBS is the most common storage for EC2 instances that need persistent storage.<br>- Works with both Linux and Windows OS types.<br>- With io1 and io2, provisioned IOPS ensure high performance for demanding workloads. | - Instance store is often used for caching, temporary data, and applications that can tolerate data loss on instance termination.<br>- Best suited for Linux-based applications, not Windows.<br>- High-performance storage for specific use cases (e.g., buffer or cache). |


