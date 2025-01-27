# Data Stores

## Data Store Options
| Feature                | [Databases](#databases)                           | [In-Memory Data Stores](#data-stores)         |
|------------------------|-------------------------------------|-------------------------------|
| **Primary Purpose**    | Persistent storage                 | Fast, temporary caching       |
| **Query Language**     | SQL / NoSQL                        | Key-value lookups             |
| **Data Durability**    | Strong persistence                 | Optional or none              |
| **Latency**            | Disk I/O dependent                 | Sub-millisecond               |
| **Scalability**        | Vertical scaling, sharding*         | Horizontal scaling            |

\* Sharding is a database partitioning technique that splits data across multiple servers (or nodes) to improve performance, scalability, and reliability. Each shard contains a subset of the total data, and together, the shards form the complete dataset.

### Databases
| Database Type      | SQL (Relational)                                      | NoSQL (Non-relational)                               |
| -------------------| ----------------------------------------------------- | --------------------------------------------------- |
| **Query Language** | SQL (Structured Query Language)                       | Varies (e.g., MongoDB Query Language, Cassandra CQL, DynamoDB API) |
| **Characteristics** | - Structured data with defined schemas.              | - Flexible schema for unstructured or semi-structured data. |
|                   | - Best for relationships between data (e.g., foreign keys). | - Highly scalable, supporting horizontal scaling.   |
|                   | - Supports ACID (Atomicity, Consistency, Isolation, Durability) transactions for reliability. | - High performance and high functionality for large-scale, high-velocity workloads. |
|**Implementations**| MySQL, [PostgreSQL](./PostgreSQL/README.md), Oracle DB, Microsoft SQL Server. | MongoDB, Cassandra, DynamoDB, Couchbase. |
| **Data examples** | Tables                                              | Key-value, in-memory, document, graph, json |

### In-Memory Data Store / Caching solutions
| Feature                | Redis                                 | Memcached                          |
|------------------------|---------------------------------------|------------------------------------|
| **Data Structures**    | Rich (strings, hashes, lists, sets)  | Key-value only                    |
| **Persistence**        | Optional (snapshotting/AOF)          | None                              |
| **Replication**        | Supports replication                 | No replication support            |
| **Clustering**         | Yes                                  | No                                |
| **Pub/Sub Messaging**  | Yes                                  | No                                |
| **Use Case**           | Complex caching, real-time analytics | Simple, high-performance caching  |
