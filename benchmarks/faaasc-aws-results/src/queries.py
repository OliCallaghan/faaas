from boto3 import client
from datetime import datetime, timedelta
from botocore.endpoint import threading
from pandas import DataFrame
from pandas.core.groupby.groupby import Callable

logs_client = client('logs')

q_count = 0
p_count = 0

def start_invoc_query(logGroupName: str, time_start: datetime, time_end: datetime):
    global q_count
    q_count += 1

    return logs_client.start_query(
        logGroupName=logGroupName,
        startTime=int(time_start.timestamp()),
        endTime=int(time_end.timestamp()),
        queryString="""
            fields @timestamp, @message
                | filter @message like /REPORT/
                | stats count(@message) as invocations
        """,
    )

def start_duration_query(logGroupName: str, time_start: datetime, time_end: datetime):
    global q_count
    q_count += 1

    return logs_client.start_query(
        logGroupName=logGroupName,
        startTime=int(time_start.timestamp()),
        endTime=int(time_end.timestamp()),
        queryString="""
            fields @timestamp, @message
                | filter @message like /Billed Duration/
                | parse @message "Billed Duration: * ms" as billedDuration
                | parse @message "Memory Size: * MB" as memorySize
                | filter ispresent(billedDuration)
                | stats stddev(billedDuration) as stdBilledDuration, avg(billedDuration) as avgBilledDuration, sum(billedDuration) as sumBilledDuration by memorySize
        """,
    )

def start_local_continuation_duration_query(logGroupName: str, time_start: datetime, time_end: datetime):
    global q_count
    q_count += 1

    return logs_client.start_query(
        logGroupName=logGroupName,
        startTime=int(time_start.timestamp()),
        endTime=int(time_end.timestamp()),
        queryString="""
            fields @timestamp, @message
                | filter @message like /Local continuation of:/
                | parse @message "Local continuation of: * took:" as continuationId
                | parse @message "took: * ms" as duration
                | filter ispresent(duration)
                | stats count(duration) as countLocalInvocations, stddev(duration) as stdLocalDuration, avg(duration) as avgLocalDuration, sum(duration) as sumLocalDuration by continuationId
        """,
    )

def start_compute_continuation_estimate_duration_query(logGroupName: str, time_start: datetime, time_end: datetime):
    global q_count
    q_count += 1

    return logs_client.start_query(
        logGroupName=logGroupName,
        startTime=int(time_start.timestamp()),
        endTime=int(time_end.timestamp()),
        queryString="""
            fields @timestamp, @message
                | filter @message like /Proxy saving estimated to save:/
                | parse @message "Proxy saving estimated to save: * USD" as saving
                | parse @message "USD for *\n" as continuationId
                | filter ispresent(saving)
                | stats stddev(saving) as stdEstimatedSaving, avg(saving) as avgEstimatedSaving, sum(saving) as sumEstimatedSaving by continuationId
        """,
    )

def start_compute_continuation_estimate_overhead_query(logGroupName: str, time_start: datetime, time_end: datetime):
    global q_count
    q_count += 1

    return logs_client.start_query(
        logGroupName=logGroupName,
        startTime=int(time_start.timestamp()),
        endTime=int(time_end.timestamp()),
        queryString="""
            fields @timestamp, @message
                | filter @message like /Proxy saving computation took:/
                | parse @message "Proxy saving computation took: * ms" as estimationTime
                | parse @message "ms for: *\n" as continuationId
                | filter ispresent(estimationTime)
                | stats stddev(estimationTime) as stdEstimationTime, avg(estimationTime) as avgEstimationTime, sum(estimationTime) as sumEstimationTime by continuationId
        """,
    )

def start_query_publish_to_amq_time(logGroupName: str, time_start: datetime, time_end: datetime):
    global q_count
    q_count += 1

    return logs_client.start_query(
        logGroupName=logGroupName,
        startTime=int(time_start.timestamp()),
        endTime=int(time_end.timestamp()),
        queryString="""
            fields @timestamp, @message
                | filter @message like /Publish to MQ time:/
                | parse @message "Publish to MQ time: * ms" as publishTime
                | filter ispresent(publishTime)
                | stats stddev(publishTime) as stdPublishTime, avg(publishTime) as avgPublishTime, sum(publishTime) as sumPublishTime
        """,
    )

def await_query_results(query_response):
    from time import sleep

    global p_count

    print(f"Executing query {p_count}/{q_count}")
    p_count += 1

    query_id = query_response["queryId"]
    results = logs_client.get_query_results(queryId=query_id)

    while results["status"] != "Complete":
        sleep(1)
        results = logs_client.get_query_results(queryId=query_id)

    return results

def query_result_to_df(results, default_cols: list[str], fill_zeros: bool = False):
    data = results["results"]

    if len(data) == 0:
        data = [[0 for _ in default_cols]] if fill_zeros == True else []
        return DataFrame(columns=default_cols, data=data) # type: ignore

    return DataFrame(
        data=[[cell["value"] for cell in row] for row in data],
        columns=[str(cell["field"]) for cell in data[0]] # type: ignore
    )
