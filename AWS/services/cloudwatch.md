# CloudWatch
Amazon CloudWatch is a monitoring and observability service offered by AWS that provides actionable insights into your AWS resources and applications. 

---

# Add here a note
> [[Note]]
> By default Cloudwatch will update its metrics every 5 minutes. However, it can be reduced down to one minute, by enabling detailed monitoring. Which has some cost.

## Metrics
Tracks performance metrics from AWS services (e.g. EC2, RDS, etc)

## Events

## Logs

---

## Example
Set an alarm for EC2 when CPU utilization is greater than 60% for 5 minutes.
1. Go to `Cloudwatch`
2. `All alarms`
3. `Create alarm`
4. `Select Metric`
5. `EC2`
6. IMPORTANT! Make sure that Instance ID is correct!!!
7. Select instance `CPUUtilization`
8. Set `Static` threshold type
9. Set the value
10. For period set `5 minutes`
11. `Next`
12. Set `In alarm`
13. Define the notification: `Select existing topic`
14. Send a notification to `MonitoringTeam`
15. Add also for the example a EC2 action

---
---

## CLI

List Log Groups
`aws logs describe-log-groups`

Get Log Events
`aws logs get-log-events --log-group-name my-log-group --log-stream-name my-log-stream`
