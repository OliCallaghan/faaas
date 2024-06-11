from boto3 import client
from datetime import datetime, timedelta
from pandas import DataFrame, concat

from model_aws import aws_invocation_cost, aws_invocation_rate, model_aws_billed_time

logs_client = client('logs')

def start_invoc_query(logGroupName: str, time_start: datetime, time_end: datetime):
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
                | stats sum(billedDuration) as totalBilledDurationMs by memorySize
        """,
    )

def await_query_results(query_response):
    query_id = query_response["queryId"]
    results = logs_client.get_query_results(queryId=query_id)

    while results["status"] == "Running":
        results = logs_client.get_query_results(queryId=query_id)

    return results

time_start = datetime.now() - timedelta(minutes=15)
time_end = datetime.now()

functionNames = ["pets-local", "pets-proxy", "pets-adaptive"]
functionLogGroups = [f"/aws/lambda/{functionName}" for functionName in functionNames]

query_responses = [(
    start_invoc_query(logGroup, time_start, time_end),
    start_duration_query(logGroup, time_start, time_end)
) for logGroup in functionLogGroups]

def query_result_to_df(results):
    data = results["results"]

    return DataFrame(
        data=[[cell["value"] for cell in row] for row in data],
        columns=[str(cell["field"]) for cell in data[0]] # type: ignore
    )

query_dfs = [(
    query_result_to_df(await_query_results(invoc_resp)),
    query_result_to_df(await_query_results(duration_resp))
) for invoc_resp, duration_resp in query_responses]

for invoc_df, duration_df in query_dfs:
    print(invoc_df)
    print(duration_df)

billing_dfs = [invoc_df.join(duration_df, how="outer") for invoc_df, duration_df in query_dfs]
billing_df = concat(billing_dfs)
billing_df["functionName"] = functionNames

# Cast to integer types
billing_df["invocations"] = billing_df["invocations"].astype(int)
billing_df["totalBilledDurationMs"] = billing_df["totalBilledDurationMs"].astype(int)
billing_df["memorySize"] = billing_df["memorySize"].astype(int)

billing_df["invocationCost"] = billing_df["invocations"] * aws_invocation_cost
billing_df["billedRate"] = billing_df["memorySize"] * aws_invocation_rate / 1024
billing_df["billedCost"] = billing_df["billedRate"] * billing_df["totalBilledDurationMs"] / 1e3

billing_df["totalCost"] = billing_df["invocationCost"] + billing_df["billedCost"]

print(billing_df[["totalCost", "invocationCost", "billedCost"]])

import seaborn as sns
import matplotlib as mpl

col_width = 2.96289

tex = True
if tex: mpl.use("pgf")

import matplotlib.pyplot as plt
sns.set_theme()

# Melt the DataFrame to have a long-form DataFrame suitable for seaborn
melted_df = billing_df.melt(id_vars=["functionName"], value_vars=["billedCost", "invocationCost"], var_name="CostType", value_name="Cost")

if tex: mpl.rcParams.update({
    "pgf.texsystem": "pdflatex",
    'font.family': 'serif',
    'font.size' : 8,
    'text.usetex': True,
    'pgf.rcfonts': False,
})

# Create the stacked bar chart
fig, ax = plt.subplots(figsize=(col_width, col_width * 1.5))

billing_breakdown_df = billing_df[["invocationCost", "billedCost", "functionName"]].copy()
billing_breakdown_df.set_index("functionName", inplace=True)
billing_breakdown_df.plot.bar(stacked=True, ax=ax)

# Add labels and title
plt.xlabel("Function Name")
plt.ylabel("Cost (USD)")
plt.title("Cost breakdown by function")

# Display the plot
if tex: plt.savefig("assets/aws-results.pgf", bbox_inches="tight")
else: plt.show()
