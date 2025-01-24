# Elastic Compute Cloud (EC2)

---

## Components

What need to be configured when creating a new EC2:
1. **OS:** [AMI](#1-amazon-machine-image-ami)
2. **CPU Cores:** [Instance type](#2-instance-type)
3. **Ram size:** [Instance type](#2-instance-type)
4. **Storage space:**  
   * Network attached ([EBS](#3-amazon-elastic-block-store-ebs)/EFS)
   * H/W (EC2 Instance store)
5. **Network:**
6. **Firewall:** [Security Groups](#5-security-groups)
7. **Provisioning:** (Bootstrap sequence - EC2 User Data)

---

### 1. Amazon Machine Image (AMI)
Pre-configured templates used to launch instances. AMIs include operating systems, application servers, and applications.

### 2. Instance Type
Defines the compute, memory, storage configuration, etc. of the instance, details are available [here](https://aws.amazon.com/ec2/instance-types/).

#### Instance Types
| Instance Types	| Description |
| ----------------- | ------------|
| General Purpose	| Balanced compute, memory, and networking resources. Suitable for web servers, dev/test, and small databases. Example: t2, m5.
| Compute Optimized	 | Optimized for compute-intensive workloads requiring high performance, like batch processing or HPC. Example: c5.
| Memory Optimized	| Designed for memory-intensive tasks, such as in-memory databases and real-time big data analytics. Example: r5.
| Accelerated Computing	| Leverages GPUs or FPGAs for compute-intensive applications, such as ML, AI, or video processing. Example: g4dn, p3.
| Storage Optimized |	High, sequential read/write access to large datasets, such as NoSQL databases or big data workloads. Example: i3.
| High Performance Computing (HPC)	|Purpose-built for HPC with low-latency networking and enhanced scalability. Example: hpc6id.

> [!NOTE]
> Visit [https://instances.vantage.sh/](https://instances.vantage.sh/) to compare EC2 instances.

```mermaid
---
title: Instance Families & Types relationship
---
flowchart TD
next((" "))
next2((" "))
next3((" "))

subgraph types2["Compute Optimized"]
    direction LR
    family21["'C5'"]
    family22["'C6g'"]
    next21((" "))
    next22((" "))
end

subgraph types1["General Purpose"]
    direction LR
    family11["'t2'"]
    family12["'t3'"]
    next11((" "))
    next12((" "))
end

subgraph example["'Instance type'"]
    direction LR
    family["'family'"]
end
```

#### Instance family examples:

| Instance family    | Purpose/Category          | vCPU | Memory (GiB) | Storage          | Network Performance     | Example Use Case                      |
|------------------|---------------------------|------|--------------|------------------|-------------------------|---------------------------------------|
| **t2.micro**     | General Purpose (Burstable)| 1    | 1            | EBS-Only         | Low to Moderate         | Low-traffic websites, testing         |
| **m5.large**     | General Purpose           | 2    | 8            | EBS-Only         | Up to 10 Gbps           | Databases, app servers, caching       |
| **c5.large**     | Compute Optimized         | 2    | 4            | EBS-Only         | Up to 10 Gbps           | Batch processing, web servers         |
| **r5.large**     | Memory Optimized          | 2    | 16           | EBS-Only         | Up to 10 Gbps           | High-memory apps, in-memory databases |
| **g4dn.xlarge**  | GPU-Optimized             | 4    | 16           | NVMe + EBS-Only  | Up to 25 Gbps           | Machine learning, AI inference        |
| **i3.large**     | Storage Optimized         | 2    | 15.25        | 1 x 475 NVMe SSD | Up to 10 Gbps           | High-speed storage, NoSQL databases   |

#### Naming convetion
Example: m5.2xlarge

* **m**: Instance class
* **5**: Hardware Generation
* **2xlarge**: Instance class size (higher the size --> higher the specs)

### 3. [Amazon Elastic Block Store (EBS)](./ebs.md)
Persistent network storage volumes that can be attached to EC2 instances.

> [!NOTE]
> By default, when creating an instance, the root volute has `Delete on termination` enabled.
> When creating a new volume however, this is will not enabled by default when attached to an instance.

### 4. Tag
Key-value pairs. Good for filtering and billing.

E.g.
```
Project: Dummy-application
Environment: Development
Owner: Their name
```

### 5. Security Groups
Virtual firewalls that control inbound and outbound traffic to EC2 instances.

```mermaid
flowchart LR

internet1["IP / Security Group"] -- Inbound traffic --> instance 

subgraph types2["Security Group"]
    direction TB
    instance["EC2 Instance"]
end

instance -- Outbound traffic -->  internet2["IP / Security Group"]
```

> [!IMPORTANT]
> Secury Groups contain **ONLY** allow rules
> By default, all **INBOUND** traffic is **DENIED**
> Remember **least privilege principle** is used throughtout the AWS.
> So, we only **ALLOW** what we need.
>
> On the other hand, by default all **OUTBOUND** traffic is **ALLOWED**

Example:
| Type | Protocol | Port | Source    | Description |
| ---- | ---------| -----| -------   | ------------|
| SSH  |  TCP     | 22   | 0.0.0.0/0 | Control     |

```mermaid
---
title: Relationship between Security Groups and Instances (ER Diagram)
---
erDiagram
    SecurityGroup["Security Group"] {}
    Instance["Instance"] {}
    SecurityGroup }|--|{ Instance : "many to many"
```

> [!IMPORTANT]
> Use a specific Security Group for SSH and maintain it with care!

#### Troubleshooting:
* **Connection timeout**: Related to Security Group rules
* **Connection refused**: Related to Application/Server

#### Limitations:
* Depend on region/VPC combination

### 6. Key Pairs
Secure authentication for EC2 instances. A key pair consists of a public key stored on the instance and a private key used to log in.

#### Steps:
1. Create key pairs
2. Download .pem file on the device of your desire
3. Run `chmod 400 <key-pair-file>.pem`
4. Then `ssh -i <key-pair-file.pem> <user>@<public-ip>`

> [!TIP]
> Best practice: 
> Different keys, based on the environment (e.g. different key for production, developmen, QA, etc.)

As an alternative, from the `Management Console` we can use `Instance Connect` to connect to the Instance,
without utilizing/generating a `Key Pair`. In this case though, maybe **IPv6** for `SSH` should be also enabled.
1. `EC2` menu
2. Select desired instance
3. `Connect`

#### 7. IAM Roles
> [!CAUTION]
> NEVER store Access keys in EC2 instances (or other resources), use roles instead!

Create a [role](./iam.md#roles) and attach that role to your EC2 in order to be able to access resources from it.

```mermaid
---
title: Relationship between IAM Role and EC2 (ER Diagram)
---
erDiagram
    SecurityGroup["IAM Role"] {}
    Instance["EC2"] {}
    SecurityGroup ||--|{ Instance : "one to many"
```
---

## Creating an EC2 Instance
1. Choose an AMI
2. Choose an instance type
3. Configure the Instance
4. Add storage
5. Add tags
6. Configure Security Group
7. Review

---


## CLI

List Instances
`aws ec2 describe-instances`

Start an Instance
`aws ec2 start-instances --instance-ids <i-1234567890abcdef0>`

Stop an Instance
`aws ec2 stop-instances --instance-ids <i-1234567890abcdef0>`

Terminate an Instance
`aws ec2 terminate-instances --instance-ids <i-1234567890abcdef0>`