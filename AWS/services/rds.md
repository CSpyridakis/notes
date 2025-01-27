# Relational Database Service (RDS)
Is a **managed** database service offered by AWS that simplifies setting up, operating, and scaling relational databases in the cloud. 

> [!CAUTION]
> Although it's possible to create and run databases on EC2, in this implementation, we have to **back up** and **patch** the database or to make it **highly available**, **fault-tolerant**, etc.
>
> On the other hand, **AWS RDS** will handle **backups**, **upgrades**, **system patching**, etc. for us. It will also enable [vertical and horizontal scaling](../onboarding/cloud-models.md#hybrid-cloud), as well as its [high availability](../onboarding/cloud-models.md#hybrid-cloud) features.

> [!IMPORTANT]
> For **in-memory** databases use [Elasticache](./elasticache.md).
---

## Key Features of Amazon RDS
- **Automated Backups**: Daily backups and point-in-time recovery.
- **Scaling**: Vertical and horizontal scaling to meet demand.
- **High Availability**: Multi-AZ deployments for redundancy and failover support.
- **Security**: Integration with IAM, VPC, encryption at rest, and in transit.
- **Performance Monitoring**: Tools like CloudWatch for metrics and performance insights.

> [!WARNING]
> WE CANNOT HAVE DIRECT ACCESS TO THE DATABASE (LIKE USING SSH)!

---

## Common Use Case 
For security reasons, it is common to have RDS in a private network.
```mermaid
graph LR
  
  web[Internet] -.-> elb
  style web opacity:0

  subgraph VPC
    elb(("ELB"))
    ec2-1["EC2"]
    ec2-2["EC2"]
    ec2-3["EC2"]

    elb --> ec2-1
    elb --> ec2-2
    elb --> ec2-3

    rds[(Amazon RDS DB instance)]
    
    ec2-1 <--> |Read/write| rds
    ec2-2 <--> |Read/write| rds
    ec2-3 <--> |Read/write| rds
  end 
```

## Databases Options

| **Database**       | **Overview**                                                                 | **Use Cases**                                                                                   | **Key Features**                                                                                 |
|---------------------|-----------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| **Amazon Aurora**   | Cloud-optimized DB compatible with MySQL and PostgreSQL.                   | High-performance and scalable apps, SaaS, web apps.                                           | Auto-scaling storage, multi-master capabilities, global databases, 5x faster than MySQL and 3x that Postgress        |
| **MySQL**           | Popular open-source database.                                              | Web apps, CMS (WordPress, Joomla), small to medium OLTP systems.                              | Community support, suitable for read-intensive workloads, cost-effective.                      |
| **PostgreSQL**      | Feature-rich and standards-compliant open-source database.                 | Complex queries, GIS (PostGIS), enterprise apps, data analytics.                              | Robust data integrity, extensibility, support for JSON and ANSI SQL.                           |
| **MariaDB**         | Community-driven MySQL fork focusing on performance and innovation.        | Medium-scale apps needing thread pooling, innovation-focused workloads.                       | Enhanced JSON handling, faster replication, and query execution.                               |
| **Oracle Database** | Enterprise-grade commercial database.                                      | Large-scale enterprise apps (ERP, CRM), high-security and disaster recovery requirements.     | Multi-tenant architecture, in-memory processing, advanced analytics.                          |
| **SQL Server**      | Relational database for Windows-based applications.                        | Microsoft ecosystem (.NET apps, SharePoint), transactional systems, BI and reporting needs.   | Integration with Microsoft services, Always On Availability Groups, advanced reporting tools.  |


### Aurora Serverless
Amazon Aurora is an ideal choice for applications that require high performance and scalability but have **unpredictable** or **infrequent** workloads. It eliminates the need for complex capacity planning and offers a fully managed database solution that automatically adjusts to changing demands.

Concept:
```mermaid
graph TD
  
  sharedstorage["Shared Storage"]
  style sharedstorage opacity:0

  sharedstorage <-.-> auroras1
  sharedstorage <-.-> auroras2
  sharedstorage <-.-> auroras3
  sharedstorage <-.-> auroras4
  sharedstorage <-.-> a1
  sharedstorage <-.-> a2

  subgraph auroras[" "]
    auroras1@{ shape: lin-cyl, label: "Aurora"}
    auroras2@{ shape: lin-cyl, label: "Aurora"}
    auroras3@{ shape: lin-cyl, label: "Aurora"}
    auroras4@{ shape: lin-cyl, label: "Aurora"}
    a1((" "))
    a2((" "))
  end
  style auroras opacity:0
  
  proxy["Proxy Fleet
      (Managed by Aurora)"]
  style proxy fill:#eba434

  auroras1 --> proxy
  auroras2 --> proxy
  auroras3 --> proxy
  auroras4 --> proxy
  a1 --> proxy
  a2 --> proxy

  proxy <-.-> client["Client"]
  style client opacity:0
```

- **Auto Scaling:** Automatically adjusts to meet application demands.
- **Compatible with PostgreSQL and MySQL:** Supports both databases for flexible deployment.
- **Pay-per-Second Pricing:** Charges are based on actual usage, billed per second.
- **Minimal Management Overhead:** Designed for ease of use with automatic management features.
- **No Capacity Planning Required:** Scales automatically to accommodate workload fluctuations.

---
## RDS Deployments

### RDS Replicas
Improve read performance and distribute the read workload across multiple nodes.
Can scale up to 15 nodes.

#### Read Data
Application can read from all RDS
```mermaid
architecture-beta
    group app(server)[Application]

    service rds1(database)[Replica RDS] in app
    service rdsmain(database)[Main RDS] in app
    service rds2(database)[Replica RDS] in app
    service application1(cloud)[Application] in app
    service application2(cloud)[Application] in app
    service application3(cloud)[Application] in app

    rds1:L <-- R:rdsmain
    rdsmain:L --> R:rds2

    application1:L -- R:application2
    application2:L -- R:application3

    rds1:B --> T:application1
    rdsmain:B --> T:application2
    rds2:B --> T:application3
```

#### Write Data
Application can write only in the main RDS.
```mermaid
architecture-beta
    group app(server)[Application]

    service rds1(database)[Replica RDS] in app
    service rdsmain(database)[Main RDS] in app
    service rds2(database)[Replica RDS] in app
    service application1(cloud)[Application] in app
    service application2(cloud)[Application] in app
    service application3(cloud)[Application] in app

    rds1:L <-- R:rdsmain
    rdsmain:L --> R:rds2

    application1:L -- R:application2
    application2:L -- R:application3

    application2:T --> B:rdsmain
```

### Multi-AZ
Enhance high availability (failover) and durability for production workloads. Can have only 1 AZ as failover.
```mermaid
graph LR
subgraph az1[Availability Zone 1]
  rds-main@{ shape: lin-cyl, label: "RDS main"}
end

subgraph az2[Availability Zone 2]
  rds-failover@{ shape: lin-cyl, label: "RDS failover"}
end

rds-main -.-> |Replication| rds-failover

application[Application]

rds-main --> |Read| application
application --> |Write| rds-main 

rds-failover <-.-> |Failover| application

style rds-failover fill:#bbf,stroke:#f00,stroke-width:2px,color:#000,stroke-dasharray: 10 10
style rds-main fill:#797,stroke:#f00,stroke-width:2px,color:#000,stroke-dasharray: 10 10
```

### Multi-Regions
Provide disaster recovery and improve performance for globally distributed applications. However, there are replication costs, because we need to transfer data between regions.

```mermaid
graph LR

subgraph reg2[Region2]
  direction TB
  rds-failover@{ shape: lin-cyl, label: "RDS failover"}
  app2[Application]

  rds-failover --> |Read| app2
end

subgraph reg1[Region 1]
  direction TB
  rds-main@{ shape: lin-cyl, label: "RDS main"}
  app1[Application]

  rds-main --> |Read|app1
  app1 --> |Write| rds-main
end

app2 --> |Write| rds-main

rds-main -.-> |Replication| rds-failover

style rds-failover fill:#bbf,stroke:#f00,stroke-width:2px,color:#000,stroke-dasharray: 10 10
style rds-main fill:#797,stroke:#f00,stroke-width:2px,color:#000,stroke-dasharray: 10 10
```
---

## Create a basic DB for testing
Create a DB with the following options.
- `Standard create`
- Either `MySQL` or `PostgreSQL`
- Templates: `Dev/Test` or `Free Tier`
- Availability: `Single DB instance`
- DB instance identifier: `give name`
- Credentials management: `Self managed`
- Instance configuration:
  - Burstable classes
  - `db.t3.micro`
- Storage type: either `gp3`  or `gp2`
- Allocated Storage: `20Gb`
- Connectivity:
  - `Don't connect to EC2`
  - `IPv4`

---

## CLI

List DB Instances
`aws rds describe-db-instances`

Create a DB Instance
`aws rds create-db-instance --db-instance-identifier mydb --db-instance-class db.t3.micro --engine mysql --allocated-storage 20 --master-username admin --master-user-password password`

Delete a DB Instance
`aws rds delete-db-instance --db-instance-identifier mydb --skip-final-snapshot`
