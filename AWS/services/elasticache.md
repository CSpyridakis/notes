# ElastiCache
ElastiCache is a fully managed, **in-memory** **data store** and **caching** service provided by AWS. It is designed to improve the performance of applications by enabling low-latency access to frequently accessed data. ElastiCache supports two popular open-source engines: **Redis** and **Memcached**.

Concept:
```mermaid
graph LR

web
elb(("ELB"))
ec2["EC2"]
elasticache[("ElastiCache")]
rds@{ shape: lin-cyl, label: "RDS"}

style web opacity:0


web -.-> elb
elb -->ec2

ec2 <--> |Read/Write from cache
            fast| elasticache
ec2 <--> |Read/Write from DB
            slow| rds
```