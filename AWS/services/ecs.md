# Elastic Container Service (ECS)
Fully managed container orchestration service by AWS. It helps you run and scale containerized applications using Docker containers without managing your own infrastructure.

---

One option is to run containers in **EC2** instances.
However, there is also an alternative option, to run containers using **Fargate**.

| Feature	| Fargate |	EC2 |
| ----------| -----| -------|
| Definition|	[Serverless](../onboarding/serverless.md) mode: AWS manages compute infrastructure. | User-managed mode: Full control over EC2 instances.
| Infrastructure |No EC2 instances; AWS handles provisioning. | Requires managing and provisioning EC2 instances.
| Scaling | Automatic scaling of tasks based on resource needs. | Requires manual configuration or auto-scaling setup.
| Cost Model | Pay for resources used by containers (CPU, memory).| Pay for EC2 instances (even if underutilized).
| Operational Overhead | Minimal (no OS updates, scaling, or instance setup). | High (OS updates, patching, capacity planning).
| Resource Control | Limited (CPU and memory per task, no instance choice). | Full control (instance type, size, storage, etc.).
| Networking | Directly integrated with AWS VPC (per task ENI). | Networking shared across all containers on the instance.
| Use Case Suitability | Simple or unpredictable workloads needing minimal setup. | Complex workloads needing specific instance configurations.
| Custom AMIs | Not supported (tasks run on AWS-managed infra).|  Supported (customize the EC2 AMI for specific needs).
| Performance | Overhead from abstraction; may affect high-performance apps.	| Low-level control; suitable for performance-critical apps.
| Security | Task-level isolation (separate VPC interfaces). | Instance-level isolation (shared by tasks on an instance).
| Examples | Batch jobs, microservices, quick deployments. | Stateful apps, heavy compute workloads, GPU workloads.

---

## Elastic Container Registry (ECR)
Fully managed private container registry to store, manage, and deploy Docker container images. 
It is integrated with Amazon **ECS**, [EKS](./eks.md), and other AWS services, enabling seamless container management throughout the development lifecycle.

---

## CLI

List Clusters
`aws ecs list-clusters`

List Services in a Cluster
`aws ecs list-services --cluster my-cluster`

Update a Service
`aws ecs update-service --cluster my-cluster --service my-service --desired-count 2`
