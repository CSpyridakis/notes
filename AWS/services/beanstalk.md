# Beanstalk
AWS Elastic Beanstalk is a **Platform as a Service (PaaS)** that helps developers deploy, manage, and scale applications easily without worrying about the underlying infrastructure. It supports multiple programming languages and automatically handles provisioning, load balancing, scaling, and monitoring.

> [!NOTE]
> For a full comparison between EC2 and Beanstalk, read [this](./ec2-vs-beanstalk.md).

---

| Feature | Description | 
| ------- | ----------- |
| **Fully Managed Deployment**  |   Automatically provisions and configures AWS resources like [EC2](./ec2.md), [ELB](./elb.md), Auto Scaling Groups, and [RDS](./rds.md).
| **Supports Multiple Programming Languages**  |   Works with Python, Java, Node.js, .NET, Ruby, PHP, and Go.
| **Built-in Load Balancing & Auto Scaling**  |   Manages traffic and adjusts resources based on demand.
| **Infrastructure Control**  |   You can still access and customize the underlying EC2 instances, security groups, and databases.
| **Integration with Developer Tools**  |   Supports Git, Jenkins, AWS CLI, and IDEs like Visual Studio and IntelliJ.
| **Monitoring & Logging**  |   Uses Amazon CloudWatch and AWS X-Ray for performance tracking and debugging.
| **Zero Additional Cost**  |   You pay only for the AWS resources it provisions (EC2, RDS, etc.), not the Elastic Beanstalk service itself.



