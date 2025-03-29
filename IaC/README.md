# Infrastructure as Code (IaC)

**Infrastructure as Code (IaC)** is the practice of managing and provisioning computing infrastructure using **machine-readable configuration files**, rather than through manual processes or interactive tools (like web consoles or GUIs).

With IaC, you treat infrastructure the same way you treat application code:
- Versioned in Git
- Reusable
- Testable
- Automated

> **Goal:** Automate infrastructure to ensure consistency, reproducibility, and scalability.

---

## Why IaC?

- **Speed**: Rapid provisioning of environments  
- **Consistency**: No more “it works on my machine”  
- **Repeatability**: Spin up identical environments on demand  
- **Version Control**: Infrastructure changes are tracked  
- **Collaboration**: Teams work together like in software development  
- **Disaster Recovery**: Rebuild infra quickly from code

---

## Categories of IaC Tools  
(*Based on "Terraform: Up & Running"* by Yevgeniy Brikman)

IaC tools can be grouped by how they operate and the level of abstraction they provide:

---

### 1. Scripting Tools

Use traditional programming or scripting languages (like Bash, Python, PowerShell) to execute CLI commands and automate infrastructure provisioning.

- **Style:** Imperative — tells *how* to do things step-by-step.
- **Use case:** Quick automation, glue scripts, bootstrapping.

#### Examples:
- Bash + AWS CLI
- Python + boto3
- PowerShell for Azure

---

### 2. Configuration Management Tools

Focus on managing the **state of existing systems**, like installing packages, setting permissions, and ensuring services are running.

- **Style:** Mostly declarative, with some imperative options.
- **Use case:** Post-provisioning setup and ongoing maintenance.

#### Examples:
| Tool      | Notes                                 |
|-----------|----------------------------------------|
| [**Ansible**](./Ansible/README.md) | YAML-based, agentless, very popular |
| **Chef**    | Ruby DSL, good for complex logic    |
| **Puppet**  | Declarative, good for large environments |
| **SaltStack** | Fast, scalable, combines styles    |

---

### 3. Templates

Preconfigured virtual machine or container images that include the OS, packages, and app configs.

- **Style:** Static or semi-dynamic.
- **Use case:** Fast bootstrapping, golden image strategy.

#### Examples:
- Amazon Machine Images (AMIs)
- Docker Images
- Packer templates (build images as code)

---

### 4. Orchestration Tools

Coordinate and manage complex deployments involving multiple services or systems. Define **how services work together**.

- **Style:** Declarative, usually uses YAML or JSON.
- **Use case:** Multi-service architecture, container orchestration.

#### Examples:
| Tool          | Description                        |
|---------------|------------------------------------|
| **Kubernetes** | Automates deployment and scaling of containers |
| **Docker Compose** | Defines multi-container Docker apps |
| **Nomad**     | HashiCorp tool for orchestrating applications |

---

### 5. Provisioning Tools

Handle the **creation, modification, and deletion of infrastructure resources**. Typically used to create cloud infrastructure like VMs, networks, and databases.

- **Style:** Declarative vs Imperative
  - Declarative: *What* you want (e.g., Terraform)
  - Imperative: *How* to build it (e.g., scripts)

#### Examples:
| Tool           | Style       | Notes |
|----------------|-------------|-------|
| [**Terraform**](./Terraform/README.md)  | Declarative | Cloud-agnostic, HCL syntax |
| **Pulumi**     | Declarative (multi-language) | Supports TypeScript, Python, Go |
| **CloudFormation** | Declarative | AWS-native IaC in YAML/JSON |
| **Azure Bicep** | Declarative | Azure's alternative to ARM templates |
| **Vagrant**     | Hybrid | Developer-focused VM automation |

---

## Summary

| Category                | Focus                        | Example Tools                         |
|-------------------------|------------------------------|----------------------------------------|
| **Scripts**             | CLI-based automation         | Bash, Python + AWS CLI                |
| **Config Management**   | OS/package state enforcement | Ansible, Puppet, Chef, SaltStack      |
| **Server Templates**    | Pre-built images             | AMI, Docker, Packer                   |
| **Orchestration**       | Multi-service deployment     | Kubernetes, Docker Compose, Nomad     |
| **Provisioning**        | Cloud resource provisioning  | Terraform, Pulumi, CloudFormation     |