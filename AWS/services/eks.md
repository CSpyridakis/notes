# Elastic Kubernetes Service (EKS)
A fully managed service that simplifies the deployment, management, and scaling of Kubernetes clusters on AWS. It abstracts the complexity of managing Kubernetes infrastructure while providing powerful tools for building containerized applications.

---

**EKS** can manage containers in both options that [ECS](./ecs.md) supports, i.e. **EC2** and **Fargate**. 


> [!IMPORTANT]
> Kubernetes is cloud-agnostic

---

## CLI

List Clusters
`aws eks list-clusters`

Create a Cluster
`aws eks create-cluster --name my-cluster --role-arn my-role --resources-vpc-config subnetIds=subnet-12345,subnet-67890`

Delete a Cluster
`aws eks delete-cluster --name my-cluster`