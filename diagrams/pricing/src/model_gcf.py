from math import ceil, floor, log2

gcf_billing_granularity = 1e3 # per ms
gcf_min_billing_time = 1e-1 # min 100ms
gcf_invocation_cost = 4e-7 # per invocation

# Memory (MB) to invocation rate (in $ per 100ms)
gcf_invocation_rates = {
    128: 0.000000231,
    256: 0.000000463,
    512: 0.000000925,
    1024: 0.000001650,
    2048: 0.000002900,
    4096: 0.000005800,
    8192: 0.000006800,
    16384: 0.000013600,
    32768: 0.000027200,
}
gcf_allocation_types = [mem for mem in gcf_invocation_rates.keys()] # in MB

# Return billing rate per second for allocated mem in MB
def model_gcf_billed_rate(allocated_mem: int):
    invocation_rate = gcf_invocation_rates.get(allocated_mem)

    assert invocation_rate is not None
    return invocation_rate * 10

# Return billing rate per second for allocated mem in MB
def model_gcf_billed_time(time: float, allocated_mem: int):
    time = ceil(time * gcf_billing_granularity) / gcf_billing_granularity # round up to nearest ms
    time = max(gcf_min_billing_time, time) # ensure minimum billing time
    billed_rate = model_gcf_billed_rate(allocated_mem)

    return billed_rate * time

def model_gcf_total_cost(time: float, allocated_mem: int = 128):
    billed_time = model_gcf_billed_time(time, allocated_mem)

    # add to invocation cost
    return gcf_invocation_cost + billed_time
