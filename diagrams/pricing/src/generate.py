import matplotlib as mpl
mpl.use("pgf")

import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd

sns.set_theme()

from math import ceil
from model_aws import (
    model_aws_billed_rate,
    model_aws_billed_time,
    model_aws_total_cost,
    aws_billing_granularity,
    aws_allocation_types,
    aws_invocation_cost
)
from model_azure import (
    model_azr_billed_rate,
    model_azr_billed_time,
    model_azr_total_cost,
    azr_billing_granularity,
    azr_allocation_types,
    azr_invocation_cost,
)
from model_gcf import (
    model_gcf_billed_rate,
    model_gcf_billed_time,
    model_gcf_total_cost,
    gcf_billing_granularity,
    gcf_allocation_types,
    gcf_invocation_cost,
)

def with_granularity(granularity):
    return lambda val: ceil(val * granularity) / granularity

with_aws_granularity = with_granularity(aws_billing_granularity)
with_azr_granularity = with_granularity(azr_billing_granularity)
with_gcf_granularity = with_granularity(gcf_billing_granularity)

aws_min_yield_time = lambda mem: with_aws_granularity(aws_invocation_cost / model_aws_billed_rate(mem))
azr_min_yield_time = lambda mem: with_azr_granularity(azr_invocation_cost / model_azr_billed_rate(mem))
gcf_min_yield_time = lambda mem: with_gcf_granularity(gcf_invocation_cost / model_gcf_billed_rate(mem))

aws_memory_confs = aws_allocation_types
azr_memory_confs = azr_allocation_types
gcf_memory_confs = gcf_allocation_types
memory_confs = aws_allocation_types + azr_allocation_types + gcf_allocation_types
memory_confs = range(min(memory_confs), max(memory_confs) + 1)

def nearest_mem_cfg(cfgs):
    return lambda mem: max([cfg for cfg in cfgs if cfg <= mem])

nearest_aws_memory_conf = nearest_mem_cfg(aws_memory_confs)
nearest_azr_memory_conf = nearest_mem_cfg(azr_memory_confs)
nearest_gcf_memory_conf = nearest_mem_cfg(gcf_memory_confs)

aws_min_yield_time_df = pd.DataFrame({
    "memory": memory_confs,
    "min_yield_time": [aws_min_yield_time(nearest_aws_memory_conf(mem)) for mem in memory_confs],
    "provider": "AWS Lambda",
})

azr_min_yield_time_df = pd.DataFrame({
    "memory": memory_confs,
    "min_yield_time": [azr_min_yield_time(nearest_azr_memory_conf(mem)) for mem in memory_confs],
    "provider": "Azure Functions",
})

gcf_min_yield_time_df = pd.DataFrame({
    "memory": memory_confs,
    "min_yield_time": [gcf_min_yield_time(nearest_gcf_memory_conf(mem)) for mem in memory_confs],
    "provider": "Google Cloud Functions",
})

min_yield_time_df = pd.concat([aws_min_yield_time_df, azr_min_yield_time_df, gcf_min_yield_time_df])
min_yield_time_df = min_yield_time_df[min_yield_time_df["memory"] <= 2048]

db_req_roundtrip_time_upper = 9e-2
db_req_roundtrip_time_lower = 6e-2

mpl.rcParams.update({
    "pgf.texsystem": "pdflatex",
    'font.family': 'serif',
    'font.size' : 8,
    'text.usetex': True,
    'pgf.rcfonts': False,
})

col_width = 2.96289
fig, ax = plt.subplots(figsize=(col_width * 2, col_width * 1.3))

ax = sns.lineplot(
    x="memory",
    y="min_yield_time",
    hue="provider",
    data=min_yield_time_df
)

ax.set_title("Minimum profitable yield time by function resource allocation")
ax.set_xlabel("Memory (MB)")
ax.set_ylabel("Yield time (s)")

plt.xscale("log", base=2)

handles, labels = plt.gca().get_legend_handles_labels()

db_latency = plt.axhspan(db_req_roundtrip_time_lower, db_req_roundtrip_time_upper, color='r', alpha=0.2, label='DynamoDB latency bounds')
plt.axhline(y=db_req_roundtrip_time_upper, color='r', linestyle='--')
plt.axhline(y=db_req_roundtrip_time_lower, color='r', linestyle='--')

handles.extend([db_latency])
plt.legend(handles=handles)

plt.savefig("assets/min-yield-time.pgf", bbox_inches="tight")


####################
# AWS billing cost #
####################
v_scale = 1.2
h_scale = 0.9
fig, ax = plt.subplots(figsize=(col_width * h_scale, col_width * v_scale))

billing_time_range = range(0, 150)
aws_billing_cost_128_df = pd.DataFrame({
    "time": billing_time_range,
    "billed_cost": [model_aws_total_cost(time / 1e3, 128) for time in billing_time_range],
    "memory": "128MB",
})
aws_billing_cost_256_df = pd.DataFrame({
    "time": billing_time_range,
    "billed_cost": [model_aws_total_cost(time / 1e3, 256) for time in billing_time_range],
    "memory": "256MB",
})
aws_billing_cost_512_df = pd.DataFrame({
    "time": billing_time_range,
    "billed_cost": [model_aws_total_cost(time / 1e3, 512) for time in billing_time_range],
    "memory": "512MB",
})

aws_billing_cost_df = pd.concat([
    aws_billing_cost_128_df,
    aws_billing_cost_256_df,
    aws_billing_cost_512_df,
])

ax = sns.lineplot(
    ax=ax,
    x="time",
    y="billed_cost",
    hue="memory",
    data=aws_billing_cost_df
)

ax.set_title("AWS Lambda Cost Curve")
ax.set_xlabel("Time (ms)")
ax.set_ylabel("Price (USD)")

handles, labels = plt.gca().get_legend_handles_labels()

invocation_cost = plt.axhline(y=aws_invocation_cost, color='r', linestyle='--', label='Invocation cost')
db_latency = plt.axhspan(0, aws_invocation_cost, color='r', alpha=0.2, label='Invocation cost')

handles.extend([db_latency])
plt.legend(handles=handles)

fig.savefig("assets/billed-cost-aws.pgf", bbox_inches="tight")


######################
# Azure billing cost #
######################
fig, ax = plt.subplots(figsize=(col_width * h_scale, col_width * v_scale))

billing_time_range = range(0, 350)
azr_billing_cost_128_df = pd.DataFrame({
    "time": billing_time_range,
    "billed_cost": [model_azr_total_cost(time / 1e3, 128) for time in billing_time_range],
    "memory": "128MB",
})
azr_billing_cost_256_df = pd.DataFrame({
    "time": billing_time_range,
    "billed_cost": [model_azr_total_cost(time / 1e3, 256) for time in billing_time_range],
    "memory": "256MB",
})
azr_billing_cost_512_df = pd.DataFrame({
    "time": billing_time_range,
    "billed_cost": [model_azr_total_cost(time / 1e3, 512) for time in billing_time_range],
    "memory": "512MB",
})

azr_billing_cost_df = pd.concat([
    azr_billing_cost_128_df,
    azr_billing_cost_256_df,
    azr_billing_cost_512_df,
])

ax = sns.lineplot(
    ax=ax,
    x="time",
    y="billed_cost",
    hue="memory",
    data=azr_billing_cost_df
)

ax.set_title("Azure Functions Cost Curve")
ax.set_xlabel("Time (ms)")
ax.set_ylabel("Price (USD)")

handles, labels = plt.gca().get_legend_handles_labels()

invocation_cost = plt.axhline(y=azr_invocation_cost, color='r', linestyle='--', label='Invocation cost')
db_latency = plt.axhspan(0, azr_invocation_cost, color='r', alpha=0.2, label='Invocation cost')

handles.extend([db_latency])
plt.legend(handles=handles)

fig.savefig("assets/billed-cost-azure.pgf", bbox_inches="tight")

####################
# GCF billing cost #
####################
fig, ax = plt.subplots(figsize=(col_width * h_scale, col_width * v_scale))

billing_time_range = range(0, 350)
gcf_billing_cost_128_df = pd.DataFrame({
    "time": billing_time_range,
    "billed_cost": [model_gcf_total_cost(time / 1e3, 128) for time in billing_time_range],
    "memory": "128MB",
})
gcf_billing_cost_256_df = pd.DataFrame({
    "time": billing_time_range,
    "billed_cost": [model_gcf_total_cost(time / 1e3, 256) for time in billing_time_range],
    "memory": "256MB",
})
gcf_billing_cost_512_df = pd.DataFrame({
    "time": billing_time_range,
    "billed_cost": [model_gcf_total_cost(time / 1e3, 512) for time in billing_time_range],
    "memory": "512MB",
})

gcf_billing_cost_df = pd.concat([
    gcf_billing_cost_128_df,
    gcf_billing_cost_256_df,
    gcf_billing_cost_512_df,
])

ax = sns.lineplot(
    ax=ax,
    x="time",
    y="billed_cost",
    hue="memory",
    data=gcf_billing_cost_df
)

ax.set_title("Google Cloud Functions Cost Curve")
ax.set_xlabel("Time (ms)")
ax.set_ylabel("Price (USD)")

handles, labels = plt.gca().get_legend_handles_labels()

invocation_cost = plt.axhline(y=gcf_invocation_cost, color='r', linestyle='--', label='Invocation cost')
db_latency = plt.axhspan(0, gcf_invocation_cost, color='r', alpha=0.2, label='Invocation cost')

handles.extend([db_latency])
plt.legend(handles=handles)

fig.savefig("assets/billed-cost-gcf.pgf", bbox_inches="tight")
