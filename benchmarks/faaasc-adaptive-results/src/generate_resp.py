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

response_fit_json = 'results/adaptive/responsefit.json'
profit_probb_json = 'results/adaptive/profit-prob.json'

df_r = pd.read_json(response_fit_json)
df_p = pd.read_json(profit_probb_json)

df_r["bin(1m)"] = pd.to_datetime(df_r["bin(1m)"])
df_p["bin(1m)"] = pd.to_datetime(df_p["bin(1m)"])

df_r = df_r.sort_values(by='bin(1m)')
df_p = df_p.sort_values(by='bin(1m)')

df_r.set_index("bin(1m)", inplace=True)
df_p.set_index("bin(1m)", inplace=True)

df_descision = df_r.join(df_p, how="inner")

print(df_descision)

ax = df_descision[["avgProb"]].plot(ax=ax, style="-o")
ax.lines[0].set_label("P(profit)")

ax2 = ax.twinx()
ax2.set_ylim([0, 300])
df_descision[["avgLoc"]].plot(ax=ax2, style="-x", color="green")
ax2.lines[0].set_label("Weibull3 location parameter")
ax2.set_ylabel("Weibull distribution parameter value")

start_time = pd.to_datetime("09:45")
end_time = pd.to_datetime("10:45")
interval = pd.Timedelta(minutes=30)
duration = pd.Timedelta(minutes=15)

current_time = start_time
while current_time < end_time:
    ax.axvspan(current_time, current_time + duration, color='red', alpha=0.3, label="High Latency" if current_time == start_time else "")
    ax.axvline(current_time + pd.Timedelta(minutes=10), color='blue', linestyle='--', label="High Latency for 10min" if current_time == start_time else "")
    current_time += interval

lines, labels = ax.get_legend_handles_labels()
lines2, labels2 = ax2.get_legend_handles_labels()
ax.legend(lines + lines2, labels + labels2, loc='lower center', bbox_to_anchor=(0.5, -0.9))
ax2.legend().remove()

ax.set_xlim([pd.to_datetime("10:00"), pd.to_datetime("10:40")])
plt.xlabel("Time (UTC)")
ax.set_ylabel("Split profitability probability")
plt.title("Split profitability probability over time overlayed with\nmonitor's fitted response time distribution parameters")

if tex: plt.savefig("assets/decision-mapping.pgf", bbox_inches="tight")
else: plt.show()
