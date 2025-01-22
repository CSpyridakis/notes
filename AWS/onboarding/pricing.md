# AWS Pricing

---

## CapEx (Capital Expenditures) vs OpEx (Operational Expenditures)

| Aspect	| CapEx	| OpEx |
|-------| ------| ----- |
| Nature	| Long-term investment.	| Day-to-day operational costs. |
| Accounting	| Depreciated/amortized over time.	| Fully expensed in the period. |
| Cash Flow	| Large upfront cost.	| Regular, recurring payments. |
| Examples	| Buying equipment, property.	| Rent, utilities, cloud services. |
| Tax Treatment	| Deducted over years via depreciation.	| Fully deductible in the current year. |
| Recap | Purchase servers | Pay as you go |

---

## Pricing models
### 1. On Demand
Pay for compute capacity by the hour or second with no long-term commitment

Keywords: Unpredictable workload or development/testing environments.

Example: Running a web server for a marketing campaign that may last for a few days.

### 2. Reserved
Commit to using EC2 instances for a 1- or 3-year term in exchange for a significant discount compared to On-Demand pricing

Keywords: Steady-state workloads or long-term applications.

Example: Hosting an enterprise application that runs continuously.

### 3. Spot
Use spare EC2 capacity at a much lower price than On-Demand instances. However, instances can be interrupted by AWS with two minutes of notice when capacity is needed elsewhere.

Keywords: Fault-tolerant and flexible workloads, such as big data analytics, batch processing, and CI/CD pipelines.

Example: A video rendering job that can tolerate interruptions.

### 4. Dedicated Hosts
Physical servers dedicated to your use. You gain visibility and control over the physical server, including which instances are placed on it.

Keywords: Required for regulatory or compliance needs, licensing constraints, or when you need full isolation.

Example: Running software that requires BYOL (Bring Your Own License).

---

## EC2 Pricing

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

