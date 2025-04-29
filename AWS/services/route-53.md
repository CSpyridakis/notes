# Route 53
Is a highly scalable and reliable Domain Name System (DNS) web service provided by Amazon Web Services. It connects user requests to infrastructure running on AWS—like EC2 instances, S3 buckets, or CloudFront distributions—and can also route users to infrastructure outside AWS.


## Tips:
- For intenal domain names, give values like `project.in`
- Then for each service make sure that internal resolution is enabled and you give names like `service.project.in`

---


---

## CLI

List Hosted Zones
`aws route53 list-hosted-zones`

 Create a Hosted Zone
`aws route53 create-hosted-zone --name <example.com> --caller-reference <unique-string>`

List Resource Record Sets in a Hosted Zone
`aws route53 list-resource-record-sets --hosted-zone-id <HOSTED_ZONE_ID>`

Create or Update a Resource Record Set
```
aws route53 change-resource-record-sets --hosted-zone-id <HOSTED_ZONE_ID> --change-batch '{
  "Changes": [
    {
      "Action": "UPSERT",
      "ResourceRecordSet": {
        "Name": "www.example.com",
        "Type": "A",
        "TTL": 300,
        "ResourceRecords": [
          {"Value": "192.0.2.44"}
        ]
      }
    }
  ]
}'
```

Delete a Hosted Zone
`aws route53 delete-hosted-zone --id <HOSTED_ZONE_ID>`

Create Health Checks
```
aws route53 create-health-check --caller-reference unique-string --health-check-config '{
  "IPAddress": "192.0.2.44",
  "Port": 80,
  "Type": "HTTP",
  "ResourcePath": "/",
  "RequestInterval": 30,
  "FailureThreshold": 3
}'
```

Associate VPC with Hosted Zone (Private DNS)
`aws route53 associate-vpc-with-hosted-zone --hosted-zone-id <HOSTED_ZONE_ID> --vpc VPCRegion=us-east-1,VPCId=vpc-1a2b3c4d

### Hands-On Example: 
Creating a Basic DNS Setup

Imagine you have a domain example.com and want to map it to an EC2 instance with IP 192.0.2.44:

1. Create a Hosted Zone:
`aws route53 create-hosted-zone --name <example.com> --caller-reference <unique-string>`

<unique-string> can be a UUID, to create one in Linux run `uuidgen -r`

2. Add an A Record:
```
aws route53 change-resource-record-sets --hosted-zone-id Z1D633PJN98FT9 --change-batch '{
  "Changes": [
    {
      "Action": "CREATE",
      "ResourceRecordSet": {
        "Name": "example.com",
        "Type": "A",
        "TTL": 300,
        "ResourceRecords": [
          {"Value": "192.0.2.44"}
        ]
      }
    }
  ]
}'
```

3. Verify the Record: Use the dig command or an online DNS checker to confirm the setup:
`dig example.com`

4. List zones `aws route53 list-hosted-zones`