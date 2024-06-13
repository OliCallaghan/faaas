from typing import Optional
from boto3 import client
from datetime import datetime, timedelta
from pandas import DataFrame
from redis import Redis
from scipy.stats import weibull_min
from time import sleep
from json import dumps

from os import getenv

redis_host = getenv("REDIS_HOST")
monitor_fn = getenv("MONITOR_FN")

if redis_host is None:
    raise Exception("REDIS_HOST is not set")

if monitor_fn is None:
    raise Exception("MONITOR_FN is not set")

redis_client = Redis(host=redis_host, port=6379, ssl=True, decode_responses=True)
logs_client = client('logs')

logGroupName = f"/aws/lambda/{monitor_fn}"

time_end = datetime.now()
time_start = time_end - timedelta(days=1)

polling_interval = 1

def is_finished(resp):
    return resp["status"] == "Complete" if resp is not None else False

def lambda_handler(event, context):
    resp = logs_client.start_query(
        logGroupName=logGroupName,
        startTime=int(time_start.timestamp()),
        endTime=int(time_end.timestamp()),
        queryString="""
            filter @message like /Local continuation of:/
            | parse @message "Local continuation of: * took:" as continuationId
            | parse @message "took: * ms" as duration
            | filter ispresent(duration)
            | limit 160
        """,
        )

    query_id = resp["queryId"]
    results: Optional[dict] = None

    while not is_finished(results):
        results = logs_client.get_query_results(queryId=query_id)
        sleep(polling_interval)

    if results is None:
        return

    data = results["results"]

    if len(data) == 0:
        return

    df = DataFrame(
        data=[[cell["value"] for cell in row] for row in data],
        columns=[str(cell["field"]) for cell in data[0]] # type: ignore
    )

    df["duration"] = df["duration"].astype(float)

    unique_continuation_ids = df["continuationId"].unique()

    for continuation_id in unique_continuation_ids:
        durations = df[df["continuationId"] == continuation_id]["duration"]

        shape, loc, scale = weibull_min.fit(df["duration"])

        redis_client.set(f"tasks:{monitor_fn}:{continuation_id}:params", dumps({
            "shape": float(shape), # type: ignore
            "loc": float(loc), # type: ignore
            "scale": float(scale), # type: ignore
        }))
        print(continuation_id, shape, loc, scale)
