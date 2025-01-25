# Auto Scaling Groups (ASG)

It combines its power with [Cloudwatch alarms](./cloudwatch.md) to monitor and adjust compute resources. Moreover, it can also be utilized along with [ELB](./elb.md).


**Components:**
- **Launch configuration**: This service uses Launch configuration to launch EC2 instances (see [EC2](./ec2.md)). So, make sure that you already have a launch configuration ready.

- **Scaling policies**: To determine how much the EC2 will be increased/decreased based on conditions.

- **Load Balancer**: We need to create a [Target Group](./elb.md#b-create-lb) and an [ELB](./elb.md) for this. Hence, before continuing create these two.

## Create Auto Scaling Group
1. [Create Target Group](./elb.md#a-first-create-a-target-group)
2. [Create Load Balancer](./elb.md#b-create-lb)
3. Then Create Auto Scaling Groups
    Steps:
   - Give a name
   - Provide the `launch template` you want
   - Select Availability zones
   - `Attach to an existing load balancer`
   - Turn on `ELB health checks`
   - Enable `Target tracking scaling policy`

> [[Important]]
> **DO NOT UPDATE Instances manually**. Remember, Auto Scaling will create instances based on the template provided, hence, any manual modification of the instance will be lost.

> [[Important]]
> **DO NOT STORE DATA ON INSTANCES**. Remember, Auto Scaling will create/delete instances dynamically, so, use [EFS](./efs.md) or another method to store your data.

## Strategies
| Scaling Strategy         | Description                                                                                         | Example Use Case                                       | Key Differentiator                                   |
|---------------------------|-----------------------------------------------------------------------------------------------------|-------------------------------------------------------|----------------------------------------------------|
| **1. Manual Scaling**        | You manually adjust the desired capacity of the ASG through the console, CLI, or SDK.              | Scaling up instances during a one-time traffic spike. | Fully user-controlled, no automation.              |
| **2. Dynamic Scaling**       | Adjusts the number of instances automatically based on policies and metrics.                                                                      |
|   2.a Simple/Step Scaling     | Adds or removes instances when a specified alarm threshold is breached (e.g., CPU > 70%).          | Scale out when CPU usage exceeds 70%.                | Reacts to metric thresholds with step adjustments. |
|   2.b Target Tracking Scaling | Maintains a target value for a specific metric (e.g., keep CPU at 50%).                            | Keep CPU utilization at 50% for consistent workload. | Continuously adjusts to achieve a specific metric target. |
|   2.c Scheduled Scaling       | Launches or terminates instances based on a defined schedule (e.g., time of day or week).          | Scale out every day at 9 AM for peak business hours.  | Time-based scaling, independent of metrics.        |
| **3. Predictive Scaling**    | Uses machine learning to predict future traffic and scales accordingly, proactively managing capacity. | Scale up for Black Friday sales based on traffic forecasts. | Proactively anticipates demand with ML insights.   |
