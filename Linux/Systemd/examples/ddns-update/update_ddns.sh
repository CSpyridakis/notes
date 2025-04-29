#!/bin/bash

# ----------------------------------------------
# Log IP
# ----------------------------------------------

# Define log file
LOG_DIR="/log"
LOG_FILE="$LOG_DIR/ip_logs.txt"

# Create log directory if it doesn't exist
mkdir -p "$LOG_DIR"

# Get public IP
PUBLIC_IP=$(curl -s https://api.ipify.org)

# Get current timestamp
TIMESTAMP=$(date "+%Y-%m-%d %H:%M:%S")

# Log the timestamp and IP
echo "$TIMESTAMP - $PUBLIC_IP" >> "$LOG_FILE"

# ----------------------------------------------
# Update DDNS
# ----------------------------------------------
# FIXME: This is specific to each DDNS used