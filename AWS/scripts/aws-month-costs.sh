#!/bin/bash

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
# Dates
# =====================================================================================
start_date=$(date -u +'%Y-%m-01')
end_date=$(date -u -d "$(date +'%Y-%m-01') +1 month -1 day" +'%Y-%m-%d')

# =====================================================================================
# Current cost
# =====================================================================================
total_cost_for_this_month=0
function get_current_costs(){
    # Fetch current and estimated billing data
    response=$(aws ce get-cost-and-usage \
        --time-period Start=$start_date,End=$end_date \
        --granularity MONTHLY \
        --metrics "UnblendedCost" \
        --group-by Type=DIMENSION,Key=SERVICE \
        --query "ResultsByTime[].Groups[].{Service:Keys[0], CurrentCost:Metrics.UnblendedCost.Amount}" \
        --output json)

    # Check if the response is empty
    if [[ -z "$response" ]]; then
        echo "No billing data found."
        exit 1
    fi
    

    # Print the results
    echo 
    echo_y "=================================================================="
    echo_y "  Current Billing Per Service for ($start_date to $end_date):"
    echo_y "=================================================================="
    total_cost=0
    current_month_total=0

    for row in $(echo "$response" | jq -r '.[] | @base64'); do
        _jq() {
            echo ${row} | base64 --decode | jq -r ${1}
        }

        # Extract and print Service name and CurrentCost
        service_name=$(_jq '.Service')
        current_cost=$(_jq '.CurrentCost')

        echo " - Service: [$service_name] | Current Cost: [$current_cost]"
        current_month_total=$(echo "$current_month_total + $current_cost" | bc)
    done
    echo_g "Total Current Month Cost: $(echo ${current_month_total} | awk '{printf "%.4f", $0}') \$"

    # Accumulate for total cost estimation
    total_cost_for_this_month=$(echo "$total_cost_for_this_month + $current_month_total" | bc)
}


# =====================================================================================
# Current cost
# =====================================================================================
get_current_costs

echo
echo
echo_r "=================================================================="
echo_r "TOTAL COST FOR ALL REGIONS ($start_date to $end_date): \
[$(echo ${total_cost_for_this_month} | awk '{printf "%.4f", $0}')] \$"
echo_r "=================================================================="