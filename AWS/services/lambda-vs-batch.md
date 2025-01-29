# [Lambda](./lambda.md) vs [Batch](./batch.md)

| Feature |  Batch | AWS Lambda
| --------| -------| -----------
| Execution Model | Executes batch jobs with defined start and end points. | Executes code in response to events (serverless).
| Use Case | Ideal for running large-scale, long-running batch jobs that process data in chunks. | Best for short-lived tasks or microservices that run on-demand in response to events.
| Job Duration | Can handle long-running jobs, from minutes to hours or more. | Limited to 15 minutes per invocation.
| Scaling | Automatically scales compute resources based on job demand. | Automatically scales based on the number of events.
| Resource Management | Requires setting up and managing compute environments (e.g., EC2 instances). | Fully managed, with no need to handle the underlying infrastructure.
| Compute Resources | Uses EC2 instances or Fargate for job execution. | Runs in a fully managed, serverless environment with no need to provision resources.
| Cost Structure | Pay for EC2 instance usage, including spot instances, and job execution duration. | Pay per request and per compute time (duration).
| Event-Driven | Not event-driven; batch jobs are typically scheduled or triggered manually. | Event-driven, triggered by events like HTTP requests, file uploads, or CloudWatch events.
| Complexity | Suited for complex workflows with dependencies, multi-step processes, and large data processing. | Ideal for simple, stateless functions with minimal dependencies.
| State Management | Can manage state over longer job executions using custom setups. | Stateless functions by design; state management must be external (e.g., DynamoDB, S3).
| Languages | Supports multiple languages via custom environments (Python, Java, etc.). | Supports multiple languages out-of-the-box (Node.js, Python, Java, Go, etc.).
| Container Support | Can run jobs inside Docker containers, providing flexibility and consistency. | Supports running functions in containers (via Lambda container images).