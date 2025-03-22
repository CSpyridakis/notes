# Docker Swarm

Swarm in build-in Orchestration, a server clustering solution.

## Manager - Worker Nodes
Inside the swarm there are manager and worker nodes.
- Each one could be a VM or a physical host.
- Managers store the configuration
- Multiple managers can communicate together
- Managers can also be workers. Actually is a worker with swarm control privileges.
- A Manager can become a worker and vice versa.
- Only 1 Leader can exist at a time in the network.

```mermaid
%%{init: {'theme':'neutral'}}%%
flowchart LR
    subgraph worker1["Worker"]
        direction LR
        TLS21["TLS"]
    end
    
    subgraph manager1["Manager"]
        direction LR
        TLS1["TLS"]
        CA1[Certificate Authority]
    end

    worker1 <--> manager1
```

## Swarm concept

Each **service** can have one or more **tasks**, then each **task** will handle each **container**.

```mermaid
%%{init: {'theme':'neutral'}}%%
flowchart LR
    subgraph manager["Manager"]
        direction LR

        subgraph service["service"]
            direction LR
            TLS21["N Replicas"]
        end
    end
    
    subgraph Workers[" "]
        direction LR

        subgraph node1["Available Node"]
            direction TB
            task1["Task"]
            container1["Container"]
        end

        subgraph node2["Available Node"]
            direction TB
            task2["Task"]
            container2["Container"]
        end

        subgraph node3["Available Node"]
            direction TB
            task3["Task"]
            container3["Container"]
        end
    end
    style Workers stroke-width:0px, fill:#fff;

    manager --> node1
    manager --> node2
    manager --> node3
```

## Setup Process
1. Inspect the output of `docker info`. Is '**Swarm**' enabled?
2. To initialize the Swarm run `docker swarm init`. Run it from the Leader node.
   1. This will create in the background root Certificates
3. To add a worker in the swarm, run the command that is provided after step 2 on the desired machine. The command should look like this:
   `docker swarm join --token <your-token> <ip-or-domain>:<port>`
4. By running `docker node ls` we can verify that the Leader node an the proper worker nodes exist in the network.