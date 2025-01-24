# AWS Pricing

## Pricing model
***Pay as you go*** model

| Service | Pay For | 
| ------- | ---| 
| Compute | Compute Time |
| Storage | Data stored |
| Networking | Data transfer OUT --> of the cloud* |
| IPv4      | $0.005/hour of Public IPv4** |

\* **Data entering the Cloud is FREE**
\** Free for Free Tier but only for EC2 and only for 750 hours per month (ELD, RDS, etc have charges)

> [!TIP]
> To monitor IP addresses:
>
> Search `IP Address Manager`
> From there you are able to find public IP addresses 


### Charges

**No charges for:**
- Key pairs
- Security Groups

**Charges occur for:**
- Instances
- Elastic IPs
- Volumes
- Snapshots
- Load Balancers

---

## Pricing Options

| no | **Pricing Option**        | **Advantages**                                                                                                                                  | **Disadvantages**                                                                                                                                                                 | **Limitations**                                                                                                                                                                   | **Extra Info**                                                                                                                                                                    | **Examples**                                                                                                                                                      | **Paid/Free**                                                                                                                                      | **Mandatory Things to Know**                                                                                                                                        |
| -- |---------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 1  | **On Demand**              | - Pay only for the resources you use.<br>- No upfront commitment.<br>- Flexible, scalable, and simple.                                           | - Higher cost compared to Reserved and Saving Plans.<br>- No cost savings with long-term use.                                                   | - No cost savings for long-term use.<br>- Best for unpredictable workloads.                                                                   | Ideal for startups, small businesses, and testing environments where demand is unpredictable.                                                 | - Temporary applications.<br>- Development environments.<br>- Websites with variable traffic.                                                  | Free                                                                                                                                             | Focuses on innovation, not long-term stability.                                                                                                                 |
| 2  | **Reserved**               | - Significant cost savings (up to 75%) compared to On Demand.<br>- Flexible options (1-year or 3-year term).<br>- Reserved instances can be modified (instance type, region, etc.). | - Requires upfront payment or commitment.<br>- Less flexible if usage patterns change.<br>- Longer commitment term (1 or 3 years).                                            | - Cannot be used for unpredictable or short-term workloads.<br>- Commitment must be made based on instance type and region.                                                    | Can be applied to EC2 instances, RDS, Redshift, and more.                                                                   | - Applications with predictable workloads.<br>- Long-running services.<br>- Dev/Prod environments with steady demand.                                                         | Paid                                                                                                                                            | CentOS Stream is a **rolling release** that precedes RHEL.                                                                                                      |
| 3  | **Saving Plans**           | - Flexible pricing model for various workloads.<br>- Provides up to 72% savings over On Demand.<br>- Applies to multiple services (e.g., EC2, Lambda, Fargate).                       | - Requires commitment (1 or 3 years).<br>- Can be more complex to manage than Reserved instances.                                    | - Commitment is still required (even if usage patterns change).<br>- Available only for specific services.                                | More flexible than Reserved instances and can be applied to different instance families and regions.                                    | - Varied workloads across EC2, Lambda, or Fargate.<br>- Applications that have long-term but flexible usage patterns.                    | Paid                                                                                                                                            | Requires commitment for long-term savings.                                                                                                                       |
| 4  | **Spot Instance**          | - Great cost savings (up to 90%) compared to On Demand.<br>- Ideal for non-critical, flexible workloads.                                         | - Instances can be terminated by AWS with little notice (2 minutes).<br>- Not suitable for applications that require high availability.                                           | - Availability depends on excess capacity.<br>- Unpredictable termination risk.                                                   | Ideal for stateless, fault-tolerant workloads that can be restarted easily.                                                   | - Big Data processing.<br>- CI/CD workloads.<br>- Rendering tasks.                                                              | Free                                                                                                                                             | Instances can be terminated with little notice.                                                                                                                |
| 5  | **Dedicated Hosts**        | - Full control over EC2 instances and physical servers.<br>- Can bring existing software licenses (BYOL).<br>- More compliance options for workloads.                               | - Higher cost compared to other pricing models.<br>- Requires management of physical hardware.                                       | - Cannot be used for all workloads.<br>- Can be cost-prohibitive for small or medium-sized businesses.                              | Suitable for workloads that require full server isolation or compliance needs.                                                     | - Legacy applications requiring physical isolation.<br>- Applications with stringent compliance or licensing requirements.         | Paid                                                                                                                                            | Requires significant investment in management.                                                                                                               |
| 6  | **Dedicated Instances**    | - Instances run on physically isolated hardware.<br>- Can share the underlying hardware with other AWS accounts (compared to Dedicated Hosts).                                    | - Higher cost compared to standard On Demand.<br>- Less control over the physical server than Dedicated Hosts.                                    | - Not suitable for all applications.<br>- Limited ability to manage hardware.                                                       | Ideal for applications that require physical isolation but do not need specific host control.                                        | - Workloads that require isolation from other tenants but don't need specific host control.                                           | Paid                                                                                                                                            | Ideal for compliance-heavy applications.                                                                                                                        |
| 7  | **Capacity Reservations**  | - Ensures capacity is available in specific Availability Zones.<br>- Can be combined with On Demand or Reserved instances.                                                                 | - Does not offer cost savings like Reserved or Saving Plans.<br>- Capacity reservation is for specific Availability Zones and regions.                                       | - Does not offer cost savings like Reserved or Saving Plans.<br>- Capacity reservation is for specific Availability Zones and regions.                                       | Guarantees capacity in specific regions but no cost savings.                                                             | - Critical applications that must run at specific times.<br>- Regulatory compliance workloads. | Free                                                                                                                                             | Guarantees capacity in specific regions but no cost savings.                                                                                                  |

Example
| **Pricing Model**        | **Hourly Cost Range** (t3.micro)    | **Discount (%)**  |
|--------------------------|------------------------------------|-------------------|
| **On Demand**            |                             |                |
| **Reserved (1-year)**    |                 |              |
| **Reserved (3-year)**    |                   |              |
| **Spot Instance**        |                  |                |
| **Dedicated Host**       |                            |               |
| **Dedicated Instance**   |                           |                |
| **Capacity Reservation** |                            |                |


---

