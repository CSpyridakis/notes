# CloudFormation
AWS CloudFormation is an **Infrastructure as Code (IaC)** service that allows users to **define** and **provision** AWS resources in an **automated**, **repeatable**, and **consistent manner**. It enables you to create and manage AWS infrastructure using declarative templates, written in **JSON** or **YAML**.

---

| Feature        | Description    |
| -------------- | -------------- |
| **Declarative Configuration** | Instead of scripting infrastructure manually, you **describe what resources** should be created, and AWS handles the provisioning automatically.
| **Stack Management** | A CloudFormation Stack is a collection of AWS resources managed as a **single unit**.
|                | You can update, delete, or rollback entire stacks efficiently.
| **Automatic Dependency Management** | CloudFormation **understands dependencies between resources** and provisions them in the correct order (e.g., creating an S3 bucket before attaching a policy).
| **Change Sets for Safe Updates** | **Before** applying changes, you can **preview** how they will impact existing infrastructure, helping to prevent accidental disruptions.
| **Supports AWS and Third-Party Resources** | CloudFormation supports **most** * AWS services and allows custom resource definitions.
| **Drift Detection** | Detects manual changes to infrastructure outside of CloudFormation and alerts you to potential inconsistencies.
| **Cross-Account and Cross-Region Deployment** | Allows you to deploy infrastructure **across multiple** AWS **accounts** and **regions**.
| **Cost** | **Tagged resources** per stack: a. easily **estimate** resources **cost**, b. **saving strategies** (e.g. automate addition/deletion of resources at given times)

\* If a resource is not supported we can use `custom resources`.

> [!IMPORTANT]
> **CloudFormation** comes together with **Infrastructure Composer** and **Application Composer** (a tool for the visual representation of stacks).


**CloudFormation vs Terraform:**
| Feature | AWS CloudFormation | Terraform  |
| ------| -------------------| -----------|
| Language | YAML/JSON | HCL (HashiCorp)
| Multi-Cloud | AWS-only | Multi-cloud
| State Management | Managed by AWS | Requires Backend
| Ease of Use | Simple, but verbose | More flexible


**CloudFormation yaml example:**
```yaml
---
AWSTemplateFormatVersion: '2010-09-09'
Description: Dummy example

Resources:
    MyEC2Instance:
        Type: AWS::EC2::Instance
        Properties:
            AvailabilityZone: us-west-1a    # Replace with your AZ
            ImageId: ami-01345678912345678  # Replace with a valid AMI ID for your region
            InstanceType: t2.micro
```

---

## CLI

List Stacks
`aws cloudformation list-stacks`

Create a Stack
`aws cloudformation create-stack --stack-name my-stack --template-body file://template.yaml`

Delete a Stack
`aws cloudformation delete-stack --stack-name my-stack`