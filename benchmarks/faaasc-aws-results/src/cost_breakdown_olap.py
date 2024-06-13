from boto3 import client
from datetime import datetime, timedelta
from pandas import concat

from model_aws import aws_invocation_cost, aws_invocation_rate, model_aws_billed_time
from queries import start_invoc_query, start_duration_query, await_query_results, query_result_to_df

def generate_cost_breakdown_olap(time_start: datetime, time_end: datetime):
    functionNames = [
        # "warehouse-order-http",
        "warehouse-order-local",
        "warehouse-order-proxy",
        "warehouse-order-adaptive",
        # "warehouse-report-http",
        "warehouse-report-local",
        "warehouse-report-proxy",
        "warehouse-report-adaptive"
    ]
    functionLogGroups = [f"/aws/lambda/{functionName}" for functionName in functionNames]

    query_responses = [(
        start_invoc_query(logGroup, time_start, time_end),
        start_duration_query(logGroup, time_start, time_end)
    ) for logGroup in functionLogGroups]

    query_dfs = [(
        query_result_to_df(await_query_results(invoc_resp), ["invocations"], fill_zeros=True),
        query_result_to_df(await_query_results(duration_resp), ["sumBilledDuration", "memorySize"], fill_zeros=True)
    ) for invoc_resp, duration_resp in query_responses]

    for (invoc_df, duration_df), name in zip(query_dfs, functionNames):
        print(name)
        print(invoc_df)
        print(duration_df)

    billing_dfs = [invoc_df.join(duration_df, how="outer") for invoc_df, duration_df in query_dfs]
    billing_df = concat(billing_dfs)

    billing_df["functionName"] = functionNames
    billing_df["strategy"] = [
        # "\\texttt{warehouse-order} (Baseline)",
        "\\texttt{warehouse-order} (Never Split)",
        "\\texttt{warehouse-order} (Always Split)",
        "\\texttt{warehouse-order} (Adaptive Split)",
        # "\\texttt{warehouse-report} (Baseline)",
        "\\texttt{warehouse-report} (Never Split)",
        "\\texttt{warehouse-report} (Always Split)",
        "\\texttt{warehouse-report} (Adaptive Split)",
    ]

    # Cast to integer types
    billing_df["invocations"] = billing_df["invocations"].astype(int)
    billing_df["sumBilledDuration"] = billing_df["sumBilledDuration"].astype(int)
    billing_df["memorySize"] = billing_df["memorySize"].astype(int)

    billing_df["Invocation Cost"] = billing_df["invocations"] * aws_invocation_cost
    billing_df["billedRate"] = billing_df["memorySize"] * aws_invocation_rate / 1024
    billing_df["Duration Cost"] = billing_df["billedRate"] * billing_df["sumBilledDuration"] / 1e3

    billing_df["totalCost"] = billing_df["Invocation Cost"] + billing_df["Duration Cost"]

    # Normalize the costs to the baseline
    # for function_prefix in ["warehouse-order", "warehouse-report"]:
    #     baseline_row = billing_df[billing_df["functionName"].str.endswith("-http") & billing_df["functionName"].str.contains(function_prefix)]
    #     if not baseline_row.empty:
    #         baseline_total_cost = baseline_row["totalCost"].values[0]
    #         mask = billing_df["functionName"].str.contains(function_prefix)
    #         billing_df.loc[mask, "Invocation Cost"] = billing_df[mask]["Invocation Cost"] / baseline_total_cost
    #         billing_df.loc[mask, "Duration Cost"] = billing_df[mask]["Duration Cost"] / baseline_total_cost

    import seaborn as sns
    import matplotlib as mpl

    col_width = 2.96289

    tex = True
    if tex: mpl.use("pgf")

    import matplotlib.pyplot as plt
    sns.set_theme()

    # Melt the DataFrame to have a long-form DataFrame suitable for seaborn
    melted_df = billing_df.melt(id_vars=["strategy"], value_vars=["Duration Cost", "Invocation Cost"], var_name="CostType", value_name="Cost")

    if tex: mpl.rcParams.update({
        "pgf.texsystem": "pdflatex",
        'font.family': 'serif',
        'font.size' : 8,
        'text.usetex': True,
        'pgf.rcfonts': False,
    })

    # Create the stacked bar chart
    fig, ax = plt.subplots(figsize=(col_width  * 0.8, col_width * 1.5))

    billing_breakdown_df = billing_df[["Invocation Cost", "Duration Cost", "strategy"]].copy()
    billing_breakdown_df.set_index("strategy", inplace=True)
    billing_breakdown_df.plot.bar(stacked=True, ax=ax).legend(loc='lower center', bbox_to_anchor=(0.5, -1.1))

    # Add labels and title
    plt.xlabel("Function splitting strategy")
    plt.ylabel("Cost (USD)")
    plt.title("Cost breakdown for each splitting\nstrategy by OLAP function workload")

    # Display the plot
    if tex: plt.savefig("assets/aws-strategy-breakdown-olap.pgf", bbox_inches="tight")
    else: plt.show()
