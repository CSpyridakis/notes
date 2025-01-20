#!/bin/bash

# Script to detect AWS resources 
# (EC2, S3, EBS, ELB, RDS, CloudWatch, Lambda) 
# across a single region.

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
REGION_TO_SCAN=
function get_regions() {
    regions=$(aws ec2 describe-regions --query "Regions[].RegionName" --output text)
    REGION_TO_SCAN=${regions}

    echo_y "==================================="
    echo_y "-------- REGIONS THAT EXIST -------"
    echo_y "==================================="
    for region in ${REGION_TO_SCAN}; do
        echo "- ${region}"
    done

    # Provide regions to test
    echo
    read -p "Give region to test: (default value 'us-east-1')" response
    response="${response:-us-east-1}"
    REGION_TO_SCAN=${response}
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
    echo "> Region: $REGION_TO_SCAN"
    response=$(aws ec2 describe-instances --region "$REGION_TO_SCAN" --query "Reservations[].Instances[].InstanceId" --output text)
    for resp in ${response}; do
        echo_r "  * ${resp}"
    done
}

# Function to list S3 buckets
function list_s3_buckets() {
    echo
    echo_y "============================"
    echo_y "-------- S3 Buckets --------"
    echo_y "============================"
    echo "> Region: $REGION_TO_SCAN"
    response=$(aws s3api list-buckets --query "Buckets[].Name" --output text)
    for resp in ${response}; do
        echo_r "  * ${resp}"
    done
}

# Function to list active EBS volumes
function list_ebs_volumes() {
    echo
    echo_y "============================"
    echo_y "-------- EBS Volumes -------"
    echo_y "============================"
    echo "> Region: $REGION_TO_SCAN"
    # aws ec2 describe-volumes --region "$REGION_TO_SCAN" --query "Volumes[?State == 'in-use'].VolumeId" --output text
    response=$(aws ec2 describe-volumes --region "$REGION_TO_SCAN" --query "Volumes[].VolumeId" --output text)
    for resp in ${response}; do
        echo_r "  * ${resp}"
    done
}

# Function to list EBS snapshots
function list_ebs_snapshots() {
    echo 
    echo_y "============================"
    echo_y "------- EBS Snapshots ------"
    echo_y "============================"
    echo "> Region: $REGION_TO_SCAN"
    response=$(aws ec2 describe-snapshots --region "$REGION_TO_SCAN" --owner-ids self --query "Snapshots[].SnapshotId" --output text)
    for resp in ${response}; do
        echo_r "  * ${resp}"
    done
}

# Function to list active ELBs
function list_elbs() {
    echo
    echo_y "============================"
    echo_y "-- Elastic Load Balancers --"
    echo_y "============================"
    echo "> Region: $REGION_TO_SCAN"
    response=$(aws elb describe-load-balancers --region "$REGION_TO_SCAN" --query "LoadBalancerDescriptions[].LoadBalancerName" --output text)
    for resp in ${response}; do
        echo_r "  * ${resp}"
    done
}

# Function to list active RDS instances
function list_rds_instances() {
    echo 
    echo_y "============================"
    echo_y "------ RDS Instances -------"
    echo_y "============================"
    echo "> Region: $REGION_TO_SCAN"
    response=$(aws rds describe-db-instances --region "$REGION_TO_SCAN" --query "DBInstances[].DBInstanceIdentifier" --output text)
    for resp in ${response}; do
        echo_r "  * ${resp}"
    done
}

# Function to list CloudWatch alarms
function list_cloudwatch_alarms() {
    echo 
    echo_y "============================"
    echo_y "----- CloudWatch Alarms ----"
    echo_y "============================"
    echo "> Region: $REGION_TO_SCAN"
    response=$(aws cloudwatch describe-alarms --region "$REGION_TO_SCAN" --query "MetricAlarms[].AlarmName" --output text)
    for resp in ${response}; do
        echo_r "  * ${resp}"
    done
}

# Function to list CloudWatch log groups
function list_cloudwatch_logs() {
    echo 
    echo_y "============================"
    echo_y "-- CloudWatch Log Groups ---"
    echo_y "============================"
    echo "> Region: $REGION_TO_SCAN"
    response=$(aws logs describe-log-groups --region "$REGION_TO_SCAN" --query "logGroups[].logGroupName" --output text)
    for resp in ${response}; do
        echo_r "  * ${resp}"
    done
}

# Function to list Lambda functions
function list_lambda_functions() {
    echo 
    echo_y "============================"
    echo_y "----- Lambda Functions -----"
    echo_y "============================"
    echo "> Region: $REGION_TO_SCAN"
    response=$(aws lambda list-functions --region "$REGION_TO_SCAN" --query "Functions[].FunctionName" --output text)
    for resp in ${response}; do
        echo_r "  * ${resp}"
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