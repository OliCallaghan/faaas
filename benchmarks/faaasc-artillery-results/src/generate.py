import matplotlib as mpl
from pandas import DataFrame, concat

tex = True
if tex: mpl.use("pgf")

import matplotlib.pyplot as plt
import seaborn as sns

if tex: mpl.rcParams.update({
    "pgf.texsystem": "pdflatex",
    'font.family': 'serif',
    'font.size' : 8,
    'text.usetex': True,
    'pgf.rcfonts': False,
})

suites = [
    ("pets", "pets"),
    ("warehouse/order-from-supplier", "wh-o"),
    ("warehouse/pricing-summary-report", "wh-r")
]

strategies = [("proxy", "Always"), ("adaptive", "Adaptive"), ("local", "Never")]

def load_response_times(suite_name, strategy):
    from json import load
    with open(f"node_modules/@faaas-bench/faaasc-artillery/suites/{suite_name}/reports/{strategy}.json") as f:
        j = load(f)
        return j["aggregate"]["summaries"]["http.response_time"]

response_times = [
    load_response_times(suite, strat) | {"suite_name": suite_name, "strategy": strat_name}
    for suite, suite_name in suites
    for strat, strat_name in strategies
]

baseline_times = [
    load_response_times(suite, "http") | {"suite_name": name, "strategy": "Baseline"}
    for suite, name in suites
]

df_abs = DataFrame(response_times)
df_base = DataFrame(baseline_times)

df_abs.set_index("suite_name", inplace=True)
df_base.set_index("suite_name", inplace=True)

df_norm = concat([df_abs, df_base]).join(df_base, on="suite_name", rsuffix="_base")

df_norm["min"] = df_norm["min"] / df_norm["mean_base"]
df_norm["max"] = df_norm["max"] / df_norm["mean_base"]
df_norm["mean"] = df_norm["mean"] / df_norm["mean_base"]
df_norm["p50"] = df_norm["p50"] / df_norm["mean_base"]
df_norm["median"] = df_norm["median"] / df_norm["mean_base"]
df_norm["p75"] = df_norm["p75"] / df_norm["mean_base"]
df_norm["p90"] = df_norm["p90"] / df_norm["mean_base"]
df_norm["p95"] = df_norm["p95"] / df_norm["mean_base"]
df_norm["p99"] = df_norm["p99"] / df_norm["mean_base"]
df_norm["p999"] = df_norm["p999"] / df_norm["mean_base"]

df_norm.drop(columns=[col for col in df_norm.columns if col.endswith('_base')], inplace=True)

df_norm["delta_min"] = df_norm["mean"] - df_norm["min"]
df_norm["delta_max"] = df_norm["max"] - df_norm["mean"]

sns.set_theme()

df_norm.reset_index(inplace=True)

# Create a pivot table for plotting
pivot_table = df_norm.pivot(index='suite_name', columns='strategy', values=['mean', 'min', 'max'])

col_width = 2.96289
fig, ax = plt.subplots(figsize=(col_width  * 0.8, col_width))

pivot_table["mean"] \
    .plot(kind='bar', ax=ax) \
    .legend(loc='lower center', bbox_to_anchor=(0.5, -0.75))

plt.title('Mean response times for\neach program and strategy')
plt.xlabel('Program name')
plt.ylabel('Normalised mean response time')

if tex: plt.savefig("assets/mean_response_times.pgf", bbox_inches="tight")
else: plt.show()

df = concat([df_abs, df_base])
df.sort_values(by=["suite_name", "strategy"]).to_csv("assets/response-times.csv")
