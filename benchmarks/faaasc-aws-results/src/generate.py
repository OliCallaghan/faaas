from boto3 import client
from datetime import datetime, timedelta
from pandas import DataFrame

from model_aws import aws_invocation_cost, aws_invocation_rate, model_aws_billed_time

logs_client = client('logs')

def start_invoc_query(logGroupName: str, time_start: datetime, time_end: datetime):
    return logs_client.start_query(
        logGroupName="/aws/lambda/handler",
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
        logGroupName="/aws/lambda/handler",
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

time_start = datetime.now() - timedelta(hours=1)
time_end = datetime.now()

invoc_query_response = start_invoc_query(
    "/aws/lambda/handler",
    time_start,
    time_end)

duration_query_response = start_duration_query(
    "/aws/lambda/handler",
    time_start,
    time_end)

invoc_results = await_query_results(invoc_query_response)
duration_results = await_query_results(duration_query_response)

def query_result_to_df(results):
    data = results["results"]

    return DataFrame(
        data=[[cell["value"] for cell in row] for row in data],
        columns=[str(cell["field"]) for cell in data[0]] # type: ignore
    )

invoc_df = query_result_to_df(invoc_results)
durations_df = query_result_to_df(duration_results)

billing_df = invoc_df.join(durations_df, how="outer")
billing_df["functionName"] = ["handler"]

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

tex = False
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
plt.figure(figsize=(10, 6))
# sns.barplot(x="functionName", y="Cost", hue="CostType", data=melted_df, estimator=sum)

billing_breakdown_df = billing_df[["invocationCost", "billedCost"]]
billing_breakdown_df.plot.bar(stacked=True)

# Add labels and title
plt.xlabel("Function Name")
plt.ylabel("Cost (USD)")
plt.title("Cost breakdown of functions by invocation and billed duration")

# Display the plot
plt.show()
