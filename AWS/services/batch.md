# Batch
AWS Batch is a **fully managed** batch processing service that allows you to run batch jobs at any scale. It enables you to efficiently run and scale hundreds or thousands of batch computing jobs in the cloud, without needing to manually manage infrastructure.

> [!NOTE]
> A batch job is a type of job or task that has a defined **start** and **end** point, processing data in **discrete** chunks rather than continuously.

Batch will spawn [EC2](./ec2.md) or [Spot](#) Instances.


| Feature | Description |
| --------------- | ------------ | 
| Automatic Scaling | AWS Batch automatically provisions and scales compute resources based on the volume and resource requirements of your batch jobs.
| Managed Infrastructure | You don’t need to worry about managing servers, clusters, or queues. AWS Batch handles the infrastructure for you.
| Job Scheduling | You can submit, monitor, and manage batch jobs using the AWS Management Console, CLI, or SDKs. It also supports job dependencies and prioritization.
| Support for Docker Containers | You can use Docker containers to package and run your batch jobs, enabling a consistent environment for job execution.
| Compute Environment Selection | AWS Batch integrates with Amazon [EC2](./ec2.md) (on-demand or spot instances) and Amazon [EC2](./ec2.md) Auto Scaling groups to provide appropriate compute resources.
| Flexible Job Queues | You can define job queues for different priority levels and types of jobs.
| Cost Efficiency | Using [Spot](#) instances can help reduce the cost of running batch jobs.

> [!IMPORTANT]
> A comparison between Lambda and Batch is available [here](./lambda-vs-batch.md).
