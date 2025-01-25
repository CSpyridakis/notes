# Auto Scaling

It combines its power with [Cloudwatch alarms](./cloudwatch.md) to monitor and adjust compute resources.


**Components:**
- **Launch configuration**: This service uses Launch configuration to launch EC2 instances (see [EC2](./ec2.md)). So, make sure that you already have a launch configuration ready.

- **Scaling policies**: To determine how much the EC2 will be increased/decreased based on conditions.

- **Load Balancer**: We need to create a [Target Group](./elb.md#a-first-create-a-target-group) and a [ELB](./elb.md) for this. Hence, before continuing create these two.

## Create Auto Scaling Group
- Give a name
- Provide the launch template you want
- Select Availability zones
- `Attach to an existing load balancer`
- Turn on `ELB health checks`
- Enable `Target tracking scaling policy`

> [[Important]]
> **DO NOT UPDATE Instances manually**. Remember, Auto Scaling will create instances based on the template provided, hence, any manual modification of the instance will be lost.

> [[Important]]
> **DO NOT STORE DATA ON INSTANCES**. Remember, Auto Scaling will create/delete instances dynamically, so, use [EFS](./efs.md) or another method to store your data.
>
