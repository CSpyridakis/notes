# Amazon FSx
Amazon FSx is a **fully managed & reliable** service by AWS that provides cloud-based **file storage** with support for various file systems. Launch **high-performance 3rd party** file systems.

## 1. Amazon FSx for Windows File Server
Use in Windows-based applications requiring shared file storage. File share storage for Windows applications, such as SQL Server or shared folder access. Applications needing Active Directory integration for access control.

```mermaid
flowchart TB

subgraph dc["On-Premices Infrastructure"]
direction LR
    fsx3["Windows Client"]
end

subgraph rg["Region"]
    direction LR
    
    subgraph rgg[" "]
        direction TB

        subgraph az2["Availability Zone 2"]
        direction LR
            fsx2["FSx"]
        end

        subgraph az1["Availability Zone 1"]
        direction LR
            fsx1["FSx"]
        end

        subgraph a2[" "]
        direction LR
            fsx["EC2 Instance"]
        end

        %%style a2 opacity:0
    end
    style rgg opacity:0

    fsz["\\\fs-01234567890abcdef.sample.com\share"]

    fsx1 <--> fsz
    fsx2 <--> fsz
end

fsz <-- SMB --> fsx3
fsx <--> fsz

```

### Supported protocols:
  * SMB
  * NTFS

## 2. Amazon FSx for Lustre:
Big data analytics and workloads requiring fast, parallel file systems. High-performance computing (HPC) applications like financial simulations, genomics, or weather forecasting. Media rendering or video processing workflows that require large, fast file system access.

> [!NOTE]
> About the naming: Lustre = Linux + cluster

> [!IMPORTANT]
> When **HPC** is considered, then Lustre most propably will be the option.

```mermaid
flowchart LR

subgraph on-premices["On-Premices Infrastructure"]
direction LR
    onpremices-server["Server"]
end

subgraph region["Region"]
direction LR
    region-s3[("S3 bucket")]
    region-fsx["FSx for Lustre"]
    region-ec2-instances@{ shape: st-rect, label: "EC2 Instances"}

    region-s3 <-- link --> region-fsx
    region-ec2-instances <-- data access --> region-fsx
end

region-fsx <-- data access --> onpremices-server

```

## 3. Amazon FSx for NetApp ONTAP: