import matplotlib as mpl
mpl.use("pgf")

from json import load
from matplotlib.pyplot import figure, savefig
from pandas import DataFrame, concat, read_csv
from seaborn.objects import Plot, Bar, Stack

def node_module(module_name: str):
    return f"node_modules/{module_name}"

experiment = node_module('@faaas/perf-v8-event-loop-experiment')

def read_time(fn_name: str):
    time_df = read_csv(f"{experiment}/results/local_time/{fn_name}/time.log", sep=" ", index_col=0, header=None).T
    time_df["fn"] = [fn_name]

    return time_df

def read_artillery(name: str):
    with open(f"{experiment}/results/local_{name}/reports/report.json")as artillery_file:
        artillery_json = load(artillery_file)

        artillery_agg = artillery_json["aggregate"]
        http_summaries = artillery_agg["summaries"]

        http_rate = artillery_agg["rates"]["http.request_rate"]
        http_on_http_delete_pet = http_summaries["plugins.metrics-by-endpoint.response_time.OnHttpDeletePet"]
        http_on_http_get_pets = http_summaries["plugins.metrics-by-endpoint.response_time.OnHttpGetPets"]
        http_on_http_get_pet = http_summaries["plugins.metrics-by-endpoint.response_time.OnHttpGetPet"]
        http_on_http_put_pet = http_summaries["plugins.metrics-by-endpoint.response_time.OnHttpPutPet"]

        artillery_df = DataFrame([
            {"fn": "onHttpDeletePet"} | http_on_http_delete_pet,
            {"fn": "onHttpGetPet"} | http_on_http_get_pets,
            {"fn": "onHttpGetPets"} | http_on_http_get_pet,
            {"fn": "onHttpPutPet"} | http_on_http_put_pet,
        ])

        return artillery_df.set_index("fn").add_prefix("req_").add_suffix("_ms")

on_http_delete_pet_time_df = read_time("onHttpDeletePet")
on_http_get_pet_time_df = read_time("onHttpGetPet")
on_http_get_pets_time_df = read_time("onHttpGetPets")
on_http_put_pet_time_df = read_time("onHttpPutPet")

on_http_time_df = concat([
    on_http_delete_pet_time_df,
    on_http_get_pet_time_df,
    on_http_get_pets_time_df,
    on_http_put_pet_time_df,
])

on_http_time_df.set_index("fn", inplace=True)
on_http_time_df = on_http_time_df.join(read_artillery("time"))

on_http_time_df["req_real_ms"] = on_http_time_df["req_count_ms"] * on_http_time_df["req_mean_ms"]
on_http_time_df["req_real"] = on_http_time_df["req_real_ms"] / 1000

on_http_time_df["waiting"] = on_http_time_df["real"] - (on_http_time_df["user"] + on_http_time_df["sys"])
on_http_time_df["req_waiting"] = on_http_time_df["req_real"] - (on_http_time_df["user"] + on_http_time_df["sys"])
on_http_time_df["efficiency"] = 1 - (on_http_time_df["req_waiting"] / on_http_time_df["req_real"])

# Aliases
on_http_time_df["node"] = on_http_time_df["user"]
on_http_time_df["syscall"] = on_http_time_df["sys"]
on_http_time_df["blocked"] = on_http_time_df["req_waiting"]

on_http_time_df_melt = on_http_time_df[["node", "syscall", "blocked"]] \
    .reset_index() \
    .melt(id_vars="fn", var_name="measure", value_name="value")

mpl.rcParams.update({
    "pgf.texsystem": "pdflatex",
    'font.family': 'serif',
    'font.size' : 8,
    'text.usetex': True,
    'pgf.rcfonts': False,
})

fig = figure(figsize=(3.27791,3.27791 * 2))

Plot(on_http_time_df_melt, x="fn", y="value", color="measure") \
    .add(Bar(), Stack()) \
    .label(title="Billed execution time by phase", x="Function name", y="Time (s)", color="phase") \
    .layout(engine="tight", size=(3.27791,3.27791 * 1.5)) \
    .on(fig) \
    .plot()

ax, legend = fig.axes[0], fig.legends[0]
ax.tick_params(axis='x', labelrotation=90)

legend.set_loc("upper right")
legend.set_bbox_to_anchor((0.95,0.93))

savefig("assets/faas-profile-waiting-on-io.pgf", bbox_inches="tight")
