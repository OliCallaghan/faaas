import pandas as pd
import matplotlib as mpl

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

def to_program_id(program_name):
    if program_name == "warehouse-order":
        return "wh-o"
    if program_name == "warehouse-report":
        return "wh-r"
    if program_name == "warehouse-pred":
        return "wh-p"
    if program_name == "echoer":
        return "echo"
    return program_name


col_width = 2.96289
sns.set_theme()

df = pd.read_json('results.json')

df[['program_name', 'memory', 'strategy']] = df['@log'].str.extract(r'/aws/lambda/(?P<program_name>.+)-(?P<memory>[^-]+)-(?P<strategy>[^-]+)')

df["memory"] = pd.to_numeric(df["memory"])
df["programId"] = df["program_name"].apply(to_program_id)

print(df['program_name'])
print(df['memory'])
print(df['strategy'])

df_proxy = df[df['strategy'] == 'proxy']
df_local = df[df['strategy'] == 'local']

df_proxy.set_index(["program_name", "memory"], inplace=True)
df_local.set_index(["program_name", "memory"], inplace=True)

df = df_local.join(df_proxy, lsuffix='_local', rsuffix='_proxy')
df["totalCostRatio"] = ((df["totalCost_local"] - df["totalCost_proxy"]) / df["totalCost_local"]) * 100

print(df[["totalCost_proxy", "totalCost_local", "totalCostRatio"]])

heatmap_data = df['totalCostRatio'].unstack()

plt.figure(figsize=(col_width * 2, col_width))
sns.heatmap(heatmap_data, annot=True, fmt=".2f", cmap="RdYlGn", cbar_kws={'label': 'Total Cost Reduction'})
plt.title('Total cost reduction (\%) by applying code splitting')
plt.xlabel('Memory')
plt.ylabel('Program Name')

if tex: plt.savefig("assets/program-memory-split-saving.svg", bbox_inches="tight")
else: plt.show()

df["totalCostRatio"] = df["totalCostRatio"].round(2)
df[["totalCostRatio", "programId_local"]].to_csv("assets/program-memory-split-saving.csv")
