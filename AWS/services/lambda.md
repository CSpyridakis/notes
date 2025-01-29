# Lambda

AWS Lambda is a [serverless](../onboarding/serverless.md) compute service provided by Amazon Web Services (AWS) that lets you run code without provisioning or managing servers. It automatically scales your application by running code in response to events such as changes in data, system state, or user actions.

| Feature | Description |
| ----| ----| 
| Serverless | **No servers to manage**, automatically scales to accommodate workload.
| Event-Driven | Lambda functions are **triggered by events** (e.g., HTTP requests, changes in data, etc.).
| Time Limitations | Functions have a maximum execution time (15 minutes).
| On-Demand Execution | Functions run **only when triggered**, ensuring efficiency and cost savings.
| Auto-Scaling | Lambda **automatically handles scaling** based on incoming requests.
| Multi-Language Support: | Supports **several programming languages** (Node.js, Python, Java, C#, Go, etc.).
| Cost-Effective | **Pay-per-use** model—charged **per request** (\$0.20 per 1 million requests) and **duration** (\$1.00 per 600,000 GB-seconds).
| Cost Efficiency | **Very affordable**, making it highly popular for small and large-scale applications.
| Event Scheduling | Can be triggered by scheduled events like **CRON jobs** (using **CloudWatch Events** or **EventBridge**).
| Container Support | Lambda **supports container images** that implement the **Lambda runtime API** for custom environments.
| ECS/Fargate Preference | For **larger or more complex** containerized applications, AWS **ECS/Fargate** is often preferred over **Lambda container images**.

> [!NOTE]
> Read [API Gateway](./api-gateway.md) to create serverless apps.

> [!IMPORTANT]
> A comparison between Lambda and Batch is available [here](./lambda-vs-batch.md).

---

## CLI

List Functions
`aws lambda list-functions`

Invoke a Function
`aws lambda invoke --function-name my-function response.json`

Update Function Code
`aws lambda update-function-code --function-name my-function --zip-file fileb://function.zip`