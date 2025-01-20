#!/bin/bash

# Script to detect AWS resources 
# (EC2, S3, EBS, ELB, RDS, CloudWatch, Lambda) 
# across all regions.

# =====================================================================================
# LOGS
# =====================================================================================

CLR_RED='\033[0;31m'
CLR_GREEN='\033[0;32m'
CLR_YELLOW='\033[0;33m'
CLR_RST='\033[0m'
function echo_r(){ echo -e "${CLR_RED}$*${CLR_RST}"; }
function echo_g(){ echo -e "${CLR_GREEN}$*${CLR_RST}"; }
function echo_y(){ echo -e "${CLR_YELLOW}$*${CLR_RST}"; }

# =====================================================================================
# CONTROL + C TERMINATION
# =====================================================================================

# Array to store process IDs
pids=()

# Function to handle SIGINT
function cleanup() {
    echo
    echo "Caught Ctrl+C. Terminating active processes..."
    
    # Kill all active processes
    for pid in "${pids[@]}"; do
        if ps -p "$pid" > /dev/null 2>&1; then
            echo "Killing process $pid"
            kill -9 "$pid"
        fi
    done
    
    # Exit the script
    exit 1
}

# Trap SIGINT (Ctrl+C)
trap cleanup SIGINT

# =====================================================================================
# REGIONS
# =====================================================================================

# Function to get all AWS regions
REGIONS_TO_SCAN=
function get_regions() {
    regions=$(aws ec2 describe-regions --query "Regions[].RegionName" --output text)
    REGIONS_TO_SCAN=${regions}

    echo_y "==================================="
    echo_y "--- REGIONS THAT WILL BE TESTED ---"
    echo_y "==================================="
    for region in ${REGIONS_TO_SCAN}; do
        echo "- ${region}"
    done

    # Ask the user if they want to continue
    read -p "Do you want to test services on all these regions? (y/n, default value 'n'): " response

    # Check user's response
    if ! [[ "${response}" =~ ^[Yy]$ ]]; then
        read -p "Give specific regions to test (default value 'us-east-1', space separated): " response
        response="${response:-us-east-1}"
        REGIONS_TO_SCAN=${response}
    fi
}
get_regions

# =====================================================================================
# DETECT ACTIVE SERVICES
# =====================================================================================

# Function to list active EC2 instances
function list_ec2_instances() {
    echo
    echo_y "============================"
    echo_y "------  EC2 Instances ------"
    echo_y "============================"
    for region in ${REGIONS_TO_SCAN}; do
        echo "- Region: [$region]"
        response=$(aws ec2 describe-instances --region "$region" --query "Reservations[].Instances[].[InstanceId, State.Name]" --output text)
        [[ -z "$response" ]] && continue
        
        # Read the response line by line
        while IFS=$'\t' read -r instance_id state; do
            echo_r "  * Instance: [${instance_id}], State: [${state}]"
        done <<< "$response"
    done
}

# Function to list S3 buckets
function list_s3_buckets() {
    echo
    echo_y "============================"
    echo_y "\n------- S3 Buckets -------"
    echo_y "============================"
    echo "- Region: [ALL]"
    response=$(aws s3api list-buckets --query "Buckets[].Name" --output text)
    [[ -z "$response" ]] && continue
    
    # Read the response line by line
    while IFS=$'\t' read -r bucket_name; do
        echo_r "  * Bucket Name: [${bucket_name}]"
    done <<< "$response"
}

# Function to list active EBS volumes
function list_ebs_volumes() {
    echo
    echo_y "============================"
    echo_y "-------- EBS Volumes -------"
    echo_y "============================"
    for region in ${REGIONS_TO_SCAN}; do
        echo "- Region: [$region]"
        response=$(aws ec2 describe-volumes --region "$region" --query "Volumes[].VolumeId" --output text)
        [[ -z "$response" ]] && continue

        # Read the response line by line
        while IFS=$'\t' read -r volume_id; do
            state=$(aws ec2 describe-volumes --region "$region" --volume-ids "$volume_id" --query "Volumes[0].State" --output text)
            echo_r "  * Volume: [${volume_id}], State: [${state}]"
        done <<< "$response"
    done
}

# Function to list EBS snapshots
function list_ebs_snapshots() {
    echo 
    echo_y "============================"
    echo_y "------- EBS Snapshots ------"
    echo_y "============================"
    for region in ${REGIONS_TO_SCAN}; do
        echo "- Region: [$region]"
        response=$(aws ec2 describe-snapshots --region "$region" --owner-ids self --query "Snapshots[].[SnapshotId, State]" --output text)
        [[ -z "$response" ]] && continue

        # Read the response line by line
        while IFS=$'\t' read -r snapshot_id state; do
            echo_r "  * Snapshot: [${snapshot_id}], State: [${state}]"
        done <<< "$response"
    done
}

# Function to list active ELBs
function list_elbs() {
    echo
    echo_y "============================"
    echo_y "-- Elastic Load Balancers --"
    echo_y "============================"
    for region in ${REGIONS_TO_SCAN}; do
        echo "- Region: [$region]"
        response=$(aws elb describe-load-balancers --region "$region" --query "LoadBalancerDescriptions[].LoadBalancerName" --output text)
        [[ -z "$response" ]] && continue

        # Read the response line by line
        while IFS=$'\t' read -r load_balancer_name; do
            details=$(aws elb describe-load-balancers --region "$region" --load-balancer-name "$load_balancer_name" --query "LoadBalancerDescriptions[0].[LoadBalancerName, DNSName, VPCId, Scheme, State.Code, AvailabilityZones[]]" --output text)
            echo_r "  * Load Balancer: [${load_balancer_name}], Details: [${details}]"
        done <<< "$response"
    done
}

# Function to list active RDS instances
function list_rds_instances() {
    echo 
    echo_y "============================"
    echo_y "------ RDS Instances -------"
    echo_y "============================"
    for region in ${REGIONS_TO_SCAN}; do
        echo "- Region: [$region]"
        response=$(aws rds describe-db-instances --region "$region" --query "DBInstances[].DBInstanceIdentifier" --output text)
        [[ -z "$response" ]] && continue
        
        # Read the response line by line
        while IFS=$'\t' read -r db_instance_identifier; do
            echo_r "  * DB instance identifier: [${db_instance_identifier}]"
        done <<< "$response"
    done
}

# Function to list CloudWatch alarms
function list_cloudwatch_alarms() {
    echo 
    echo_y "============================"
    echo_y "----- CloudWatch Alarms ----"
    echo_y "============================"
    for region in ${REGIONS_TO_SCAN}; do
        echo "- Region: [$region]"
        response=$(aws cloudwatch describe-alarms --region "$region" --query "MetricAlarms[].AlarmName" --output text)
        [[ -z "$response" ]] && continue

        # Read the response line by line
        while IFS=$'\t' read -r alarm_name; do
            echo_r "  * Alarm name: [${alarm_name}]"
        done <<< "$response"
    done
}

# Function to list CloudWatch log groups
function list_cloudwatch_logs() {
    echo 
    echo_y "============================"
    echo_y "-- CloudWatch Log Groups ---"
    echo_y "============================"
    for region in ${REGIONS_TO_SCAN}; do
        echo "- Region: [$region]"
        response=$(aws logs describe-log-groups --region "$region" --query "logGroups[].logGroupName" --output text)
        [[ -z "$response" ]] && continue

        # Read the response line by line
        while IFS=$'\t' read -r logGroupName; do
            echo_r "  * Log Group name: [${logGroupName}]"
        done <<< "$response"
    done
}

# Function to list Lambda functions
function list_lambda_functions() {
    echo 
    echo_y "============================"
    echo_y "----- Lambda Functions -----"
    echo_y "============================"
    for region in ${REGIONS_TO_SCAN}; do
        echo "- Region: [$region]"
        response=$(aws lambda list-functions --region "$region" --query "Functions[].FunctionName" --output text)
        [[ -z "$response" ]] && continue
        
        # Read the response line by line
        while IFS=$'\t' read -r function_name; do
            echo_r "  * Function name: [${function_name}]"
        done <<< "$response"
    done
}

# ==========================================================================
# Main execution
# ==========================================================================

list_ec2_instances
list_s3_buckets
list_ebs_volumes
list_ebs_snapshots
list_elbs
list_rds_instances
list_cloudwatch_alarms
list_cloudwatch_logs
list_lambda_functions