from boto3 import client
from datetime import datetime, timedelta
from pandas import concat

from model_aws import aws_invocation_cost, aws_invocation_rate, model_aws_billed_time
from queries import start_invoc_query, start_duration_query, await_query_results, query_result_to_df

def generate_cost_breakdown(time_experiment: datetime):
    time_start = time_experiment - timedelta(minutes=10)
    time_end = time_experiment

    functionNames = ["pets-http", "pets-local", "pets-proxy", "pets-adaptive"]
    functionLogGroups = [f"/aws/lambda/{functionName}" for functionName in functionNames]

    query_responses = [(
        start_invoc_query(logGroup, time_start, time_end),
        start_duration_query(logGroup, time_start, time_end)
    ) for logGroup in functionLogGroups]

    query_dfs = [(
        query_result_to_df(await_query_results(invoc_resp), ["invocations"], fill_zeros=True),
        query_result_to_df(await_query_results(duration_resp), ["sumBilledDuration", "memorySize"], fill_zeros=True)
    ) for invoc_resp, duration_resp in query_responses]

    billing_dfs = [invoc_df.join(duration_df, how="outer") for invoc_df, duration_df in query_dfs]
    billing_df = concat(billing_dfs)

    billing_df["strategy"] = ["AWS Lambda\n(Baseline)", "Never Split", "Always Split\n(2 Invocations)", "Adaptive Split"]

    # Cast to integer types
    billing_df["invocations"] = billing_df["invocations"].astype(int)
    billing_df["sumBilledDuration"] = billing_df["sumBilledDuration"].astype(int)
    billing_df["memorySize"] = billing_df["memorySize"].astype(int)

    billing_df["Invocation Cost"] = billing_df["invocations"] * aws_invocation_cost
    billing_df["billedRate"] = billing_df["memorySize"] * aws_invocation_rate / 1024
    billing_df["Duration Cost"] = billing_df["billedRate"] * billing_df["sumBilledDuration"] / 1e3

    billing_df["totalCost"] = billing_df["Invocation Cost"] + billing_df["Duration Cost"]

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
    fig, ax = plt.subplots(figsize=(col_width, col_width * 1.5))

    billing_breakdown_df = billing_df[["Invocation Cost", "Duration Cost", "strategy"]].copy()
    billing_breakdown_df.set_index("strategy", inplace=True)
    billing_breakdown_df.plot.bar(stacked=True, ax=ax).legend(loc='lower center', bbox_to_anchor=(0.5, -0.6))

    # Add labels and title
    plt.xlabel("Function splitting strategy")
    plt.ylabel("Cost (USD)")
    plt.title("Cost breakdown by function")

    # Display the plot
    if tex: plt.savefig("assets/aws-strategy-breakdown.pgf", bbox_inches="tight")
    else: plt.show()
