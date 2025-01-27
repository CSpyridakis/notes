# DynamoDB
Amazon DynamoDB is a **fully managed serverless NoSQL database** service offered by AWS. It is designed for **high-performance**, **scalable**, and **highly available** applications that require **low-latency** (single-digit millisecond) data access. DynamoDB is ideal for handling **large amounts** of data and provides both document and **key-value** data models. It automatically is replicated accross 3 Availability Zones.

> [!IMPORTANT]
> For **Relational** databases use [RDS](./rds.md).

| Advantage                         | Description                                                                                                                                      |
|------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| **Fully Managed**                  | DynamoDB is **fully managed**, meaning AWS handles provisioning, setup, and maintenance, eliminating the need for manual administrative tasks.       |
| **Scalability**                    | It automatically scales to meet the demands of applications with varying traffic loads, supporting **high throughput** and **large-scale** data.         |
| **Low-Latency Performance**        | Provides **sub-millisecond latency** for both reads and writes, making it ideal for real-time applications.                                           |
| **High Availability**              | DynamoDB replicates data **across multiple** availability zones to ensure high availability and durability.                                         |
| **Flexible Data Model**            | Supports both **key-value** and document data models, allowing for flexible schema design to suit various types of applications.                      |
| **Automatic Backup & Restore**     | Built-in backup and restore capabilities, allowing you to easily protect your data without additional setup.                                      |
| **Integrated Security**            | Supports encryption at rest and in transit, integrates with **AWS IAM** for access control, and provides fine-grained access policies.                 |
| **Global Tables**                  | DynamoDB allows multi-region replication with Global Tables, enabling cross-region data consistency and faster access for global applications.    |
| **On-Demand Capacity**             | Provides the flexibility to scale on-demand without requiring capacity planning, paying only for what you use.                                   |
| **Streams for Real-Time Processing**| DynamoDB Streams allow capturing and processing changes in real-time, enabling use cases like event-driven architectures and analytics.          |
| **Cost-Effectiveness**             | With on-demand capacity, you pay only for the read and write requests your application performs. You can also choose provisioned capacity for predictable workloads.|
| **Table Classes**                  | DynamoDB offers two table classes: **Standard** and **Infrequent Access**. The **Infrequent Access** class is ideal for infrequently accessed data, offering lower costs.|

Reference: https://aws.amazon.com/blogs/database/choosing-the-right-dynamodb-partition-key/
![Dynamo DB](https://d2908q01vomqb2.cloudfront.net/887309d048beef83ad3eabf2a79a64a389ab1c9f/2018/09/10/dynamodb-partition-key-1.gif)

---

## DynamoDB Accelerator - DAX 

Amazon DynamoDB Accelerator (DAX) is a fully managed, in-memory caching service designed to improve the performance of Amazon DynamoDB by providing faster read access. 

> [!CAUTION]
> **DAX** IS NOT THE SAME AS [ElastiCache](./elasticache.md).
>
> Both provide similar functionality, but DAX is specifically designed for use with DynamoDB.
> Hence, although we could use it, it is not recommended because DAX offers a 10x performance improvement.

---

## DynamoDB Global Table
This feature makes accessible tables in **low latency** from **multiple regions** due to **Active-Active** replication.

---

## CLI

1. List Tables
`aws dynamodb list-tables`

2. Describe Table
`aws dynamodb describe-table --table-name <table_name>`

3. Scans all items in a table (use carefully for large tables).
`aws dynamodb scan --table-name <table_name>`

4. Create Table
```
aws dynamodb create-table \
    --table-name <table_name> \
    --attribute-definitions AttributeName=<attribute_name>,AttributeType=<attribute_type> \
    --key-schema AttributeName=<attribute_name>,KeyType=HASH \
    --provisioned-throughput ReadCapacityUnits=<read_capacity>,WriteCapacityUnits=<write_capacity>
```
  - Common AttributeType are:
      - S for String
      - N for Number
      - B for Binary
      - Example: AttributeName=UserID,AttributeType=S means UserID is a string.
  - For now use for `read_capacity` and `write_capacity` the value `1`.

5. Delete Table
`aws dynamodb delete-table --table-name <table_name>`


6. Put Item
```
aws dynamodb put-item \
    --table-name <table_name> \
    --item '{"<attribute_name>": {"<data_type>": "<value>"}}'
```
  - Common data_type are:
      - S for String
      - N for Number
      - B for Binary

7. Get Item
```
aws dynamodb get-item \
    --table-name <table_name> \
    --key '{"<key_attribute>": {"<data_type>": "<value>"}}'
```

8. Update Item
```
aws dynamodb update-item \
    --table-name <table_name> \
    --key '{"<key_attribute>": {"<data_type>": "<value>"}}' \
    --update-expression "SET <attribute_name> = :value" \
    --expression-attribute-values '{":value": {"<data_type>": "<new_value>"}}'
```

9. Delete Item
```
aws dynamodb delete-item \
    --table-name <table_name> \
    --key '{"<key_attribute>": {"<data_type>": "<value>"}}'
```