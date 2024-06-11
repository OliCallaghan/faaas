from boto3 import client
from datetime import datetime, timedelta
from pandas import DataFrame, concat

from model_aws import aws_invocation_cost, aws_invocation_rate, model_aws_billed_time
from queries import (
    start_duration_query,
    start_local_continuation_duration_query,
    start_compute_continuation_estimate_duration_query,
    start_compute_continuation_estimate_overhead_query,
    start_query_publish_to_amq_time,
    await_query_results,
    query_result_to_df
)

functionNames = ["pets-http", "pets-adaptive", "pets-local", "pets-proxy"]
functionDisplayNames = ["AWS Lambda\n(Baseline)", "Adaptive Split", "Never Split", "Always Split\n(2 Invocations)"]

functionLogGroups = [f"/aws/lambda/{functionName}" for functionName in functionNames]

invoc_duration_cols = ["stdBilledDuration", "avgBilledDuration", "sumBilledDuration", "memorySize"]
publish_to_amq_cols = ["stdPublishTime", "avgPublishTime", "sumPublishTime"]

duration_cols = ["continuationId", "avgLocalDuration", "stdLocalDuration", "sumLocalDuration", "countLocalInvocations"]
estimate_cols = ["continuationId", "stdEstimatedSaving", "avgEstimatedSaving", "sumEstimatedSaving"]
overhead_cols = ["continuationId", "stdEstimationTime", "avgEstimationTime", "sumEstimationTime"]


def execute_queries(time_start: datetime, time_end: datetime):
    query_func_dfs = [(
        query_result_to_df(await_query_results(start_duration_query(logGroup, time_start, time_end)), invoc_duration_cols, fill_zeros=True),
        query_result_to_df(await_query_results(start_query_publish_to_amq_time(logGroup, time_start, time_end)), publish_to_amq_cols, fill_zeros=True)
    ) for logGroup in functionLogGroups]

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


def generate_continuation_breakdown(time_experiment: datetime):
    time_start = time_experiment - timedelta(minutes=10)
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

    for (query_func_df, publish_to_amq_df), name in zip(query_func_dfs, functionNames):
        query_func_df.insert(0, "functionName", name)
        query_func_df.set_index("functionName", inplace=True)

        publish_to_amq_df.insert(0, "functionName", name)
        publish_to_amq_df.set_index("functionName", inplace=True)

    query_func_dfs = [iv_df.join(pub_to_amq) for iv_df, pub_to_amq in query_func_dfs]

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

    overhead_df["stdPublishTime"] = overhead_df["stdPublishTime"].astype(float)
    overhead_df["avgPublishTime"] = overhead_df["avgPublishTime"].astype(float)
    overhead_df["sumPublishTime"] = overhead_df["sumPublishTime"].astype(float)

    overhead_df["memorySize"] = overhead_df["memorySize"].astype(int)

    overhead_compute_estimate_cols = ["sumPublishTime"] + \
        [column_by_cont_id(cont_id, "sumLocalDuration") for cont_id in continuation_ids] + \
        [column_by_cont_id(cont_id, "sumEstimationTime") for cont_id in continuation_ids]

    overhead_compute_estimate_df: DateFrame = overhead_df[["sumBilledDuration"] + overhead_compute_estimate_cols].copy() # type: ignore

    overhead_compute_estimate_df["sumOtherDuration"] = overhead_compute_estimate_df["sumBilledDuration"] - \
        sum(overhead_compute_estimate_df[col] for col in overhead_compute_estimate_cols)

    overhead_compute_estimate_df: DataFrame = overhead_compute_estimate_df[ # type: ignore
        overhead_compute_estimate_cols + ["sumOtherDuration"]
    ]

    overhead_compute_estimate_labelled_df = overhead_compute_estimate_df.copy()

    for cont_id in continuation_ids:
        overhead_compute_estimate_labelled_df[f"Estimating duration of {cont_id}"] = \
            overhead_compute_estimate_labelled_df[column_by_cont_id(cont_id, "sumEstimationTime")]
        overhead_compute_estimate_labelled_df[f"Locally executing {cont_id}"] = \
            overhead_compute_estimate_labelled_df[column_by_cont_id(cont_id, "sumLocalDuration")]

        overhead_compute_estimate_labelled_df.drop(columns=[column_by_cont_id(cont_id, "sumEstimationTime")], inplace=True)
        overhead_compute_estimate_labelled_df.drop(columns=[column_by_cont_id(cont_id, "sumLocalDuration")], inplace=True)

    overhead_compute_estimate_labelled_df["Publish to AMQ"] = overhead_compute_estimate_labelled_df["sumPublishTime"]
    overhead_compute_estimate_labelled_df["Customer JavaScript"] = overhead_compute_estimate_labelled_df["sumOtherDuration"]

    overhead_compute_estimate_labelled_df.drop(columns=["sumPublishTime"], inplace=True)
    overhead_compute_estimate_labelled_df.drop(columns=["sumOtherDuration"], inplace=True)

    overhead_compute_estimate_labelled_df["Strategy"] = overhead_compute_estimate_labelled_df.index
    overhead_compute_estimate_labelled_df["Strategy"] = overhead_compute_estimate_labelled_df["Strategy"].apply({
        name: displayName
        for name, displayName in zip(functionNames, functionDisplayNames)
    }.get)

    overhead_compute_estimate_labelled_df = overhead_compute_estimate_labelled_df.set_index("Strategy")

    import seaborn as sns
    import matplotlib as mpl

    col_width = 2.96289

    tex = True
    if tex: mpl.use("pgf")

    import matplotlib.pyplot as plt
    sns.set_theme()

    fig, ax = plt.subplots(figsize=(col_width, col_width * 1.5))

    # Plot with error bars
    overhead_compute_estimate_labelled_df.plot.bar(stacked=True, title="Continuation Breakdown", ax=ax).legend(loc='lower center', bbox_to_anchor=(0.5, -0.7))

    # Add labels and title
    plt.xlabel("Function splitting strategy")
    plt.ylabel("Duration (ms)")
    plt.title("Breakdown of duration billed section\nby function splitting strategy")

    if tex: plt.savefig("assets/aws-strategy-duration-breakdown.pgf", bbox_inches="tight")
    else: plt.show()
