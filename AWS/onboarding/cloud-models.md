# Cloud Models

## Private Cloud
On-premises

## Public Cloud

Essential characteristics of cloud computing, as defined by the National Institute of Standards and Technology (NIST):

1. On-demand, self-service
2. Broad network access
3. Multi-tenancy & Resource pooling
4. Rapid elasticity & scalability
5. Measured services

## Hybrid Cloud
Merging private and public clouds.

> [!NOTE]
>
> One of the main advantages of the cloud is **Scalability** and **High availability**.
>
> **Scalability** is related but is NOT the same to **High availability**
> 
> **Scalability** can be:
>   * **Vertical**: Increase size of resource (limit is the hardware). E.g. 4 Cores -> 8 Cores
>   * **Horizontal** (= elasticity): Increase the number of used resources. E.g. 1 CPU -> 3 CPUs
>       This means that we have to handle a distributed system.
>
> **High availability**
>   * Closely related to elasticity.
>   * Running applications in independent physical systems, so, in case of a disaster, application
>       is still running.

```mermaid
graph TD
    subgraph all[" "]
        direction RL

        %% --------------------------------------------------------------------------------
        subgraph boxdown[" "]
            direction LR
            low-number-of-instances[" "]:::clchidden -- **Horizontal**
                *Scale out* ------> high-number-of-instances[" "]:::clchidden
        end
        subgraph boxdown2[" "]
            direction RL
            low-number-of-instances2[" "]:::clchidden -- **Horizontal**
                *Scale in* ------> high-number-of-instances2[" "]:::clchidden
        end
        style boxdown width:0.01,height:0.01,opacity:0
        style boxdown2 width:0.01,height:0.01,opacity:0
        %% --------------------------------------------------------------------------------

        subgraph boxup[" "]
            direction TB

            subgraph boxup-right[" "]
                direction LR
                subgraph boxup-right-bottom[" "]
                    direction TB
                    cpu["4 cores CPU"]
                    cpu-mult@{ shape: st-rect, label: "Multiple 4 cores CPU"}

                    style cpu fill:#DAA520
                    style cpu-mult fill:#DAA520
                end
                style boxup-right-bottom width:0.01,height:0.01,opacity:0
                subgraph boxup-right-top[" "]
                    direction TB
                    cpu2["128 cores CPU"]
                    cpu2-mult@{ shape: st-rect, label: "Multiple 128 cores CPU"}

                    style cpu2 fill:#1E90FF
                    style cpu2-mult fill:#1E90FF
                end
                style boxup-right-top width:0.01,height:0.01,opacity:0
            end
            style boxup-right width:0.01,height:0.01,opacity:1

            subgraph boxup-left1[" "]
                direction BT
                low-instance-size[" "]:::clchidden -- **Vertical** 
                    *Scale up* ----> high-instance-size[" "]:::clchidden
            end
            subgraph boxup-left2[" "]
                direction TB
                low-instance-size2[" "]:::clchidden -- **Vertical** 
                    *Scale down* ----> high-instance-size2[" "]:::clchidden
            end
            style boxup-left1 width:0.01,height:0.01,opacity:0
            style boxup-left2 width:0.01,height:0.01,opacity:0
        end
        style boxup width:0.01,height:0.01,opacity:0
    end

    classDef clchidden opacity:0,width:0.01,height:0.01,padding:0,margin:0
```

## CapEx (Capital Expenditures) vs OpEx (Operational Expenditures)

| Aspect	| CapEx	| OpEx |
|-------| ------| ----- |
| Nature	| Long-term investment.	| Day-to-day operational costs. |
| Accounting	| Depreciated/amortized over time.	| Fully expensed in the period. |
| Cash Flow	| Large upfront cost.	| Regular, recurring payments. |
| Examples	| Buying equipment, property.	| Rent, utilities, cloud services. |
| Tax Treatment	| Deducted over years via depreciation.	| Fully deductible in the current year. |
| Recap | Purchase servers | Pay as you go |

## AWS Shared responsibility model
See: https://d1.awsstatic.com/security-center/Shared_Responsibility_Model_V2.59d1eccec334b366627e9295b304202faf7b899b.jpg
![Shared](https://d1.awsstatic.com/security-center/Shared_Responsibility_Model_V2.59d1eccec334b366627e9295b304202faf7b899b.jpg)