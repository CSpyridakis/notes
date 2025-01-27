# PostgreSQL

Connect to postgreSQL server
`psql -h <hostname/ip> -p <port> -U <username> `

## General commands
| Command | Description |
| ------- | ----------- |
| `\conninfo` | Show current connection information (e.g., database, user, host) |
| `\v` | Show the PostgreSQL version |
| `\l` | List all databases |
| `\du` | List all PostgreSQL roles (users) |
| `\dS` | List all system schemas |
| `\dx` | List all installed extensions |
| `\q` | Quit the `psql` session |
| `\x` | Toggle expanded display mode for query results (useful for large rows) |
| `\h` | Show help for SQL commands (e.g., `\h SELECT` for help on `SELECT`) |
| `\?` | Show help on psql command-line options |
| `\s` | Show command history of the current session |
| `\! <command>` | Execute a shell command from within `psql` (e.g., `\! ls`) |
| `\i <file_path>` | Execute SQL commands from a file |

---

## Database-Related Commands
| Command | Description |
| --------| ------------|
| `\c <database_name>` | Connect to a specified database (tab for autocomplete) |
| `\df` | List all functions in the current database |
| `\di` | List all indexes in the current database |
| `\dn` | List all schemas in the current database |
| `CREATE DATABASE <database_name>;`  | create a new database   |
| `DROP DATABASE <database_name>;`  |  Delete a Database  |
| `ALTER DATABASE <database_name> RENAME TO <new_name>;` | Rename an existing database |

---
## Table-Related Commands
| Command | Description |
| ------- | ----------- |
| `\dt` | List all tables in the current database |
| `\d <table_name>` | Describe the structure of a specific table (e.g., columns, constraints, etc.) |
| `\d+ <table_name>` | Show detailed information about a table, including storage and associated indexes |
| `\di` | List all indexes on tables in the current database |
| `CREATE TABLE <table_name> (<column_definitions>);` | Create a new table with specified columns and data types |
| `DROP TABLE <table_name>;` | Delete a table from the database |
| `ALTER TABLE <table_name> ADD COLUMN <column_definition>;` | Add a new column to an existing table |
| `ALTER TABLE <table_name> DROP COLUMN <column_name>;` | Remove a column from a table |
| `ALTER TABLE <table_name> RENAME TO <new_table_name>;` | Rename an existing table |
| `SELECT * FROM <table> ;`  |  SQL Query example  |

### Example
```
CREATE TABLE my_table (
    id SERIAL PRIMARY KEY,
    my_integer_column INT
);
```
---

## User manipulation

| Command | Description |
| --------| ------------|
|  `\du` |  List Users  |
| `CREATE ROLE <username> WITH LOGIN PASSWORD '<password>';` | Create a New User |
| `DROP ROLE <username>;` | Delete a User |
| `ALTER USER <username> WITH PASSWORD '<new_password>';` | Change the Password | 
| `ALTER ROLE <username> WITH SUPERUSER;` | Grant Superuser Privileges | 
| `ALTER ROLE <username> WITH LOGIN;` | Grant Login Privileges |
| `ALTER ROLE <username> CREATEDB;` | Grant Database Creation Privileges |
| `GRANT <privileges> ON <object> TO <username>;` | Grant Specific Privileges * to a User |
| `REVOKE <privileges> ON <object> FROM <username>;` | Revoke Specific Privileges from a User |
| `GRANT <privileges> ON TABLE <table_name> TO <username>;` | Grant specific privileges (e.g., `SELECT`, `INSERT`) on a table to a user |
| `REVOKE <privileges> ON TABLE <table_name> FROM <username>;` | Revoke specific privileges from a user on a table |
| `GRANT ALL PRIVILEGES ON DATABASE <database_name> TO <username>;` | Assign the User to a Database |
| `REVOKE ALL PRIVILEGES ON DATABASE <database_name> FROM <username>;` | Revoke all privileges from a user on a database |


\* 
**Types of Privileges**: 
  - **SELECT**: Permission to read data from a table.
  - **INSERT**: Permission to add new rows to a table.
  - **UPDATE**: Permission to modify existing data in a table.
  - **DELETE**: Permission to remove rows from a table.
  - **TRUNCATE**: Permission to truncate (delete all rows from) a table.
  - **REFERENCES**: Permission to create foreign keys referencing the table.
  - **EXECUTE**: Permission to execute a function or stored procedure.
  - **USAGE**: Permission to use a schema or sequence (useful for sequences and schemas).
  - **CREATE**: Permission to create new objects (tables, views, etc.) in a schema

**Objects in PostgreSQL**:
- **Tables**: For row-level access control (e.g., SELECT, INSERT).
- **Schemas**: For controlling the ability to create objects within a schema (e.g., CREATE).
- **Functions**: For controlling who can execute a function (e.g., EXECUTE).
- **Databases**: For granting or revoking all database-related privileges (e.g., ALL PRIVILEGES).
- **Sequences**: For controlling access to sequences used in generating unique values.

### Examples:
```
REVOKE ALL PRIVILEGES ON mytable FROM myuser;
GRANT EXECUTE ON FUNCTION myfunction() FROM myuser;
GRANT INSERT ON mytable FROM myuser;
REVOKE CREATE ON SCHEMA public FROM myuser;
```