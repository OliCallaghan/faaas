from math import floor, ceil

# Sort out the fact I've copied this from the diagrams/pricing package

aws_allocation_types = [mem for mem in range(128, 10240 + 1)]
aws_billing_granularity = 1e3 # per ms
aws_invocation_cost = 2e-7 # per invocation
aws_invocation_rate = 1.66667e-5 # per GB-s

# Return billing rate per second for allocated mem in MB
def model_aws_billed_rate(allocated_mem: int):
    # ensure allocated memory is within AWS Lambda constraints
    assert allocated_mem <= 10240
    assert allocated_mem >= 128

    # round allocated mem to nearest 1MB increment
    allocated_mem = floor(allocated_mem)
    allocated_rate = aws_invocation_rate * allocated_mem / 1024

    return allocated_rate

# Return billing rate per second for allocated mem in MB
def model_aws_billed_time(time: float, allocated_mem: int):
    time = ceil(time * aws_billing_granularity) / aws_billing_granularity # round up to nearest ms
    billed_rate = model_aws_billed_rate(allocated_mem)

    return billed_rate * time

def model_aws_total_cost(time: float, allocated_mem: int = 128):
    billed_time = model_aws_billed_time(time, allocated_mem)

    # add to invocation cost
    return aws_invocation_cost + billed_time
