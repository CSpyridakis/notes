# Infrastructure as Code (IaC) comparison solutions


## AWS CDK vs CloudFormation vs Terraform
| Feature | [AWS CDK](./cdk.md) | [CloudFormation](./cloudformation.md) | Terraform |
| --------| --------| ---------------| ----------| 
| Language | TypeScript, Python, Go, Java, C# | YAML/JSON | HCL (HashiCorp)
| Resource Management | AWS CloudFormation | AWS CloudFormation | Terraform Engine
| Modularity | High (Classes, Functions) | Low (YAML Templates) | Medium (Modules)
| Multi-Cloud | No (AWS Only) | No (AWS Only) | Yes (AWS, GCP, Azure)
| Ease of Use | High (Code-based) | Medium (Declarative) | Medium (Declarative)
| State Management | CloudFormation Manages | CloudFormation Manages | Requires Remote Backend

## AWS CDK vs CloudFormation use cases
| Use Case | AWS CDK | CloudFormation |
| ------|  -------- | ------------- |
| Programmatic logic needed (e.g., loops, conditions, dynamic naming) | ✅ Yes | ❌ No
| Quick setup & best practices | ✅ Yes | ❌ No
| Easier collaboration with DevOps & Developers | ✅ Yes | ❌ No
| Strict declarative infrastructure (No programming required) | ❌ No | ✅ Yes
