from math import ceil, floor, log2

azr_billing_granularity = 1e3 # per ms
azr_allocation_types = [128, 256, 384, 512, 640, 768, 896, 1024, 1152, 1280, 1408, 1536] # in MB
azr_min_billing_time = 1e-1 # min 100ms
azr_invocation_cost = 2e-7 # per invocation
azr_invocation_rate = 1.6e-5 # per GB-s

# Return billing rate per second for allocated mem in MB
def model_azr_billed_rate(allocated_mem: int):
    # ensure allocated memory is within azr Function constraints
    assert allocated_mem in \
        [128, 256, 384, 512, 640, 768, 896, 1024, 1152, 1280, 1408, 1536]

    # apply allocated rate
    allocated_rate = azr_invocation_rate * allocated_mem / 1024

    return allocated_rate

# Return billing rate per second for allocated mem in MB
def model_azr_billed_time(time: float, allocated_mem: int):
    time = ceil(time * azr_billing_granularity) / azr_billing_granularity # round up to nearest ms
    time = max(azr_min_billing_time, time) # ensure minimum billing time
    billed_rate = model_azr_billed_rate(allocated_mem)

    return billed_rate * time

def model_azr_total_cost(time: float, allocated_mem: int = 128):
    billed_time = model_azr_billed_time(time, allocated_mem)

    # add to invocation cost
    return azr_invocation_cost + billed_time
