# [EC2](./ec2.md) vs [Beanstalk](./beanstalk.md)


## "Traditional deployment"
```mermaid
---
title: 3-Tier Application
---
%%{init: {'theme':'neutral'}}%%
graph LR

subgraph users[" "]
direction LR
    user1(("User"))
    user2(("User"))
    user3(("User"))
end
style users opacity:0

user1 --> elb
user2 --> elb
user3 --> elb

subgraph elb-sub["Multi AZ"]
direction LR
    elb(("ELB"))
end
style elb-sub stroke:#000,stroke-width:2px,stroke-dasharray: 10 10

subgraph ec2-sub["Auto Scaling Group"]
direction LR
    subgraph ec2-sub-az1["AZ 1"]
    direction LR
        ec2-1["EC2"]
    end 
    subgraph ec2-sub-az2["AZ 2"]
    direction LR
        ec2-2["EC2"]
    end 
    subgraph ec2-sub-az3["AZ 3"]
    direction LR
        ec2-3["EC2"]
    end 
end

elb <--> ec2-1
elb <--> ec2-2
elb <--> ec2-3

subgraph db-sub[" "]
direction LR
    cache("ElastiCache")
    rds[("RDS")]
end 
style db-sub opacity:0

ec2-1 <--> cache
ec2-2 <--> cache
ec2-3 <--> cache

ec2-1 <-.-> rds
ec2-2 <-.-> rds
ec2-3 <-.-> rds
```

**PROBLEM?**
If we want to focus more on the code instead of focusing with infrastructure, we can use [Beanstalk](./beanstalk.md).

| Feature |  Elastic [Beanstalk](./beanstalk.md) |  [EC2](./ec2.md) (Manual Deployment) |  AWS [Lambda](./lambda.md)   |
| -----| ------------------- | -------------------------| --------------|
| Use Case |  Web applications with managed infra |  Full control over servers |  Serverless apps with event-driven execution
| Infrastructure Management |  Fully managed |  Manual |  Fully managed
| Auto Scaling |  Built-in |  Requires configuration |  Automatic
| Customization |  Moderate |  High |  Limited
| Supported Languages |  Python, Java, Node.js, .NET, PHP, Ruby, Go |  Any |  Any
| Pricing Model |  Pay for underlying AWS resources |  Pay for EC2 resources |  Pay per execution