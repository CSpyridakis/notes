# Lambda

AWS Lambda is a [serverless](../onboarding/serverless.md) compute service provided by Amazon Web Services (AWS) that lets you run code without provisioning or managing servers. It automatically scales your application by running code in response to events such as changes in data, system state, or user actions.

---

## CLI

List Functions
`aws lambda list-functions`

Invoke a Function
`aws lambda invoke --function-name my-function response.json`

Update Function Code
`aws lambda update-function-code --function-name my-function --zip-file fileb://function.zip`