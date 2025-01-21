#!/bin/bash
# IMPORTANT!!!
# EACH REQUEST WILL COST 0.01

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
function get_current_costs(){
    echo_r "THIS REQUEST WILL COST YOU 0.01\$."
    read -p "Are you sure you want to continue? (y/n): " response

    # Check user's response
    if ! [[ "${response}" =~ ^[Yy]$ ]]; then
        exit 0
    fi

    # Fetch current and estimated billing data
    response=$(aws ce get-cost-and-usage \
        --time-period Start=$start_date,End=$end_date \
        --granularity MONTHLY \
        --metrics "BlendedCost" "UnblendedCost" "UsageQuantity" \
        --group-by Type=DIMENSION,Key=SERVICE \
        --query "ResultsByTime[].Groups[].{
            Service:Keys[0], 
            UnblendedCost: Metrics.UnblendedCost.Amount, 
            BlendedCost: Metrics.BlendedCost.Amount, 
            UsageQuantity: Metrics.UsageQuantity.Amount
            }" \
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
    current_month_unblenced_total=0
    current_month_blenced_total=0

    for row in $(echo "$response" | jq -r '.[] | @base64'); do
        _jq() {
            echo ${row} | base64 --decode | jq -r ${1}
        }

        # Extract and print Service name and CurrentCost
        service_name=$(_jq '.Service')
        unblenced_cost=$(_jq '.UnblendedCost')
        blended_cost=$(_jq '.BlendedCost')
        usage_quantity=$(_jq '.UsageQuantity')

        echo_r " - Service: [$service_name]"
        echo "   UnblendedCost: [$unblenced_cost]"
        echo "   BlendedCost: [$blended_cost]"
        echo "   UsageQuantity: [$usage_quantity]"
        echo
        current_month_unblenced_total=$(echo "$current_month_unblenced_total + $unblenced_cost" | bc)
        current_month_blenced_total=$(echo "$current_month_blenced_total + $blended_cost" | bc)
    done
    echo_g "Total Current Unblended Month Cost: $(echo ${current_month_unblenced_total} | awk '{printf "%.4f", $0}') \$"
    echo_g "Total Current Blended Month Cost: $(echo ${current_month_blenced_total} | awk '{printf "%.4f", $0}') \$"
}

# =====================================================================================
# Current cost
# =====================================================================================
get_current_costs