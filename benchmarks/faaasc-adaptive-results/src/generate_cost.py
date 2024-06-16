import matplotlib as mpl
import pandas as pd
import seaborn as sns
import fitter as fitter
import numpy as np
from scipy.stats import weibull_min

from model_aws import model_aws_billed_time, aws_invocation_cost

col_width = 2.96289

tex = True
if tex: mpl.use("pgf")
if tex: mpl.rcParams.update({
    "pgf.texsystem": "pdflatex",
    'font.family': 'serif',
    'font.size' : 8,
    'text.usetex': True,
    'pgf.rcfonts': False,
})

import matplotlib.pyplot as plt
sns.set_theme()

fig, ax = plt.subplots(figsize=(col_width * 2, col_width * 0.8))

# Local
local_invocs_json = 'results/local/invocations.json'
local_billed_json = 'results/local/billed.json'

df_local_invocs = pd.read_json(local_invocs_json)
df_local_billed = pd.read_json(local_billed_json)

df_local_invocs["bin(1m)"] = pd.to_datetime(df_local_invocs["bin(1m)"])
df_local_billed["bin(1m)"] = pd.to_datetime(df_local_billed["bin(1m)"])

df_local_invocs = df_local_invocs.sort_values(by='bin(1m)')
df_local_billed = df_local_billed.sort_values(by='bin(1m)')

df_local_invocs.set_index("bin(1m)", inplace=True)
df_local_billed.set_index("bin(1m)", inplace=True)

df_local = df_local_invocs.join(df_local_billed, how="inner")

df_local["totalInvocCost"] = df_local["invocations"] * aws_invocation_cost

df_local["totalBilledDurationSec"] = df_local["totalBilledDurationMs"] / 1e3
df_local["totalBilledCost"] = df_local["totalBilledDurationSec"].apply(lambda sec: model_aws_billed_time(sec, 128))

df_local["totalCost"] = df_local["totalBilledCost"] + df_local["totalInvocCost"]

# Proxy
proxy_invocs_json = 'results/proxy/invocations.json'
proxy_billed_json = 'results/proxy/billed.json'

df_proxy_invocs = pd.read_json(proxy_invocs_json)
df_proxy_billed = pd.read_json(proxy_billed_json)

df_proxy_invocs["bin(1m)"] = pd.to_datetime(df_proxy_invocs["bin(1m)"])
df_proxy_billed["bin(1m)"] = pd.to_datetime(df_proxy_billed["bin(1m)"])

df_proxy_invocs = df_proxy_invocs.sort_values(by='bin(1m)')
df_proxy_billed = df_proxy_billed.sort_values(by='bin(1m)')

df_proxy_invocs.set_index("bin(1m)", inplace=True)
df_proxy_billed.set_index("bin(1m)", inplace=True)

df_proxy = df_proxy_invocs.join(df_proxy_billed, how="inner")

df_proxy["totalInvocCost"] = df_proxy["invocations"] * aws_invocation_cost

df_proxy["totalBilledDurationSec"] = df_proxy["totalBilledDurationMs"] / 1e3
df_proxy["totalBilledCost"] = df_proxy["totalBilledDurationSec"].apply(lambda sec: model_aws_billed_time(sec, 128))

df_proxy["totalCost_proxy"] = df_proxy["totalBilledCost"] + df_proxy["totalInvocCost"]

## Adaptive
adap_invocs_json = 'results/adaptive/invocations.json'
adap_billed_json = 'results/adaptive/billed.json'

df_adap_invocs = pd.read_json(adap_invocs_json)
df_adap_billed = pd.read_json(adap_billed_json)

df_adap_invocs["bin(1m)"] = pd.to_datetime(df_adap_invocs["bin(1m)"])
df_adap_billed["bin(1m)"] = pd.to_datetime(df_adap_billed["bin(1m)"])

df_adap_invocs = df_adap_invocs.sort_values(by='bin(1m)')
df_adap_billed = df_adap_billed.sort_values(by='bin(1m)')

df_adap_invocs.set_index("bin(1m)", inplace=True)
df_adap_billed.set_index("bin(1m)", inplace=True)

df_adap = df_adap_invocs.join(df_adap_billed, how="inner")

df_adap["totalInvocCost"] = df_adap["invocations"] * aws_invocation_cost

df_adap["totalBilledDurationSec"] = df_adap["totalBilledDurationMs"] / 1e3
df_adap["totalBilledCost"] = df_adap["totalBilledDurationSec"].apply(lambda sec: model_aws_billed_time(sec, 128))

df_adap["totalCost"] = df_adap["totalBilledCost"] + df_adap["totalInvocCost"]

df = df_local.join(df_adap, lsuffix="_local", rsuffix="_adaptive").join(df_proxy)

ax = df[["totalCost_local", "totalCost_proxy", "totalCost_adaptive"]].plot(ax=ax, style="-o")

ax.lines[0].set_label("Local")
ax.lines[1].set_label("Proxy")
ax.lines[2].set_label("Adaptive")

start_time = pd.to_datetime("09:45")
end_time = pd.to_datetime("10:45")
interval = pd.Timedelta(minutes=30)
duration = pd.Timedelta(minutes=15)

current_time = start_time
while current_time < end_time:
    ax.axvspan(current_time, current_time + duration, color='red', alpha=0.3, label="High Latency" if current_time == start_time else "")
    ax.axvline(current_time + pd.Timedelta(minutes=10), color='blue', linestyle='--', label="High Latency for 10min" if current_time == start_time else "")
    current_time += interval

ax.legend(loc='lower center', bbox_to_anchor=(0.5, -1))
ax.set_xlim([pd.to_datetime("10:00"), pd.to_datetime("10:40")])
plt.xlabel("Time (UTC)")
plt.ylabel("Cost (USD)")
plt.title("Cost over time of echoer workload over\nvariable latency periods by strategy")

if tex: plt.savefig("assets/split-profitability.pgf", bbox_inches="tight")
else: plt.show()
