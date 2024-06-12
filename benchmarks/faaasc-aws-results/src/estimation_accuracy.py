from boto3 import client
from datetime import datetime, timedelta
from pandas import DataFrame, concat

from model_aws import aws_invocation_cost, aws_invocation_rate, model_aws_billed_time
from queries import (
    start_duration_query,
    start_invoc_query,
    start_local_continuation_duration_query,
    start_compute_continuation_estimate_duration_query,
    start_compute_continuation_estimate_overhead_query,
    start_query_publish_to_amq_time,
    await_query_results,
    query_result_to_df
)

functionNames = ["pets-proxy", "pets-adaptive", "warehouse-order-local", "warehouse-order-adaptive", "warehouse-report-local", "warehouse-report-adaptive"]
functionDisplayNames = [
    "\\texttt{pets} (Always Split)",
    "\\texttt{pets} (Adaptive Split)",
    "\\texttt{warehouse-order} (Never Split)",
    "\\texttt{warehouse-order} (Adaptive Split)",
    "\\texttt{warehouse-report} (Never Split)",
    "\\texttt{warehouse-report} (Adaptive Split)",
]

functionLogGroups = [f"/aws/lambda/{functionName}" for functionName in functionNames]

billed_invocs_cols = ["invocations"]
billed_duration_cols = ["stdBilledDuration", "avgBilledDuration", "sumBilledDuration", "memorySize"]

duration_cols = ["continuationId", "avgLocalDuration", "stdLocalDuration", "sumLocalDuration", "countLocalInvocations"]
estimate_cols = ["continuationId", "stdEstimatedSaving", "avgEstimatedSaving", "sumEstimatedSaving"]
overhead_cols = ["continuationId", "stdEstimationTime", "avgEstimationTime", "sumEstimationTime"]


def execute_queries(time_start: datetime, time_end: datetime):
    query_func_resp = [(
        start_invoc_query(logGroup, time_start, time_end),
        start_duration_query(logGroup, time_start, time_end),
    ) for logGroup in functionLogGroups]

    query_func_dfs = [(
        query_result_to_df(await_query_results(invoc_resp), billed_invocs_cols, fill_zeros=True),
        query_result_to_df(await_query_results(duration_resp), billed_duration_cols, fill_zeros=True)
    ) for invoc_resp, duration_resp in query_func_resp]

    query_responses = [(
        start_local_continuation_duration_query(logGroup, time_start, time_end),
        start_compute_continuation_estimate_duration_query(logGroup, time_start, time_end),
        start_compute_continuation_estimate_overhead_query(logGroup, time_start, time_end)
    ) for logGroup in functionLogGroups]

    query_cont_dfs = [(
        query_result_to_df(await_query_results(duration_resp), duration_cols),
        query_result_to_df(await_query_results(estimate_resp), estimate_cols),
        query_result_to_df(await_query_results(overhead_resp), overhead_cols),
    ) for duration_resp, estimate_resp, overhead_resp in query_responses]

    return query_func_dfs, query_cont_dfs


def generate_estimation_accuracy(time_experiment: datetime):
    time_start = time_experiment - timedelta(minutes=20)
    time_end = time_experiment

    query_func_dfs, query_cont_dfs = execute_queries(time_start, time_end)

    for duration_df, estimate_df, overhead_df in query_cont_dfs:
        duration_df.set_index("continuationId", inplace=True, drop=False) # keep one
        estimate_df.set_index("continuationId", inplace=True)
        overhead_df.set_index("continuationId", inplace=True)

    query_cont_dfs = [
        duration_df.join(estimate_df).join(overhead_df)
        for duration_df, estimate_df, overhead_df in query_cont_dfs
    ]

    for query_cont_df, name in zip(query_cont_dfs, functionNames):
        query_cont_df.insert(0, "functionName", name)
        query_cont_df.set_index(["functionName", "continuationId"], inplace=True)

    for (query_invoc_df, query_duration_df), name in zip(query_func_dfs, functionNames):
        query_invoc_df.insert(0, "functionName", name)
        query_invoc_df.set_index("functionName", inplace=True)

        query_duration_df.insert(0, "functionName", name)
        query_duration_df.set_index("functionName", inplace=True)

    query_func_dfs = [query_invoc_df.join(query_duration_df) for query_invoc_df, query_duration_df in query_func_dfs]

    query_func_df = concat(query_func_dfs)
    query_cont_df = concat(query_cont_dfs)

    query_cont_df["stdEstimatedSaving"] = query_cont_df["stdEstimatedSaving"].astype(float)
    query_cont_df["avgEstimatedSaving"] = query_cont_df["avgEstimatedSaving"].astype(float)
    query_cont_df["sumEstimatedSaving"] = query_cont_df["sumEstimatedSaving"].astype(float)

    query_cont_df["stdEstimationTime"] = query_cont_df["stdEstimationTime"].astype(float)
    query_cont_df["avgEstimationTime"] = query_cont_df["avgEstimationTime"].astype(float)
    query_cont_df["sumEstimationTime"] = query_cont_df["sumEstimationTime"].astype(float)

    query_cont_df["stdLocalDuration"] = query_cont_df["stdLocalDuration"].astype(float)
    query_cont_df["avgLocalDuration"] = query_cont_df["avgLocalDuration"].astype(float)
    query_cont_df["sumLocalDuration"] = query_cont_df["sumLocalDuration"].astype(float)
    query_cont_df["countLocalInvocations"] = query_cont_df["countLocalInvocations"].astype(int)

    query_cont_df = query_cont_df.reset_index()
    query_cont_df = query_cont_df.pivot(index="functionName", columns="continuationId", values=[
        "stdEstimatedSaving",
        "avgEstimatedSaving",
        "sumEstimatedSaving",
        "stdEstimationTime",
        "avgEstimationTime",
        "sumEstimationTime",
        "stdLocalDuration",
        "avgLocalDuration",
        "sumLocalDuration",
        "countLocalInvocations",
    ])

    continuation_ids = set(continuation_id for _, continuation_id in query_cont_df.columns)

    def column_by_cont_id(continuation_id: str, column: str):
        return f"{continuation_id}_{column}"

    query_cont_df.columns = [column_by_cont_id(cont_id, col) for col, cont_id in query_cont_df.columns]
    query_cont_df = query_cont_df.fillna(0)

    overhead_df = query_func_df.join(query_cont_df, how="outer")
    overhead_df = overhead_df.fillna(0)

    overhead_df["stdBilledDuration"] = overhead_df["stdBilledDuration"].astype(float)
    overhead_df["avgBilledDuration"] = overhead_df["avgBilledDuration"].astype(float)
    overhead_df["sumBilledDuration"] = overhead_df["sumBilledDuration"].astype(int)

    overhead_df["invocations"] = overhead_df["invocations"].astype(int)
    overhead_df["memorySize"] = overhead_df["memorySize"].astype(int)

    ## ADDED THIS
    overhead_df["billedRate"] = overhead_df["memorySize"] * aws_invocation_rate / 1024
    overhead_df["Invocation Cost"] = overhead_df["invocations"] * aws_invocation_cost
    overhead_df["Duration Cost"] = overhead_df["billedRate"] * overhead_df["sumBilledDuration"] / 1e3

    efficiency_df: DataFrame = overhead_df[["Invocation Cost", "Duration Cost"]].copy() # type: ignore

    for cont_id in continuation_ids:
        efficiency_df[f"Calculated {cont_id} overhead"] = abs(overhead_df[column_by_cont_id(cont_id, "sumEstimatedSaving")])

    efficiency_df["Strategy"] = efficiency_df.index
    efficiency_df["Strategy"] = efficiency_df["Strategy"].apply({
        name: displayName
        for name, displayName in zip(functionNames, functionDisplayNames)
    }.get)

    efficiency_df = efficiency_df.set_index("Strategy")

    import seaborn as sns
    import matplotlib as mpl

    col_width = 2.96289

    tex = True
    if tex: mpl.use("pgf")

    import matplotlib.pyplot as plt
    sns.set_theme()

    fig, ax = plt.subplots(figsize=(col_width, col_width * 1.5))

    # Plot with error bars
    efficiency_df.plot.bar(stacked=True, ax=ax).legend(loc='lower center', bbox_to_anchor=(0.5, -0.65))

    ax.patches[4].set_hatch("//")

    # Add labels and title
    plt.xlabel("Function splitting strategy")
    plt.ylabel("Cost (USD)")
    plt.title("Overhead calculation\nby function splitting strategy")

    if tex: plt.savefig("assets/aws-adaptive-estimate-accuracy.pgf", bbox_inches="tight")
    else: plt.show()
