import plotly.express as px
import plotly.io as pio
import pandas as pd
import tikzplotly

df = pd.DataFrame([
    dict(Task="Invocation 1", Start="2020-01-01 00:00:00.000", Finish="2020-01-01 00:00:00.250", Resource="MicroVM A"),
    dict(Task="Invocation 2", Start="2020-01-01 00:00:00.100", Finish="2020-01-01 00:00:00.450", Resource="MicroVM B"),
    dict(Task="Invocation 3", Start="2020-01-01 00:00:00.300", Finish="2020-01-01 00:00:00.550", Resource="MicroVM A"),
    dict(Task="Invocation 4", Start="2020-01-01 00:00:00.400", Finish="2020-01-01 00:00:00.650", Resource="MicroVM C"),
    dict(Task="Invocation 5", Start="2020-01-01 00:00:00.500", Finish="2020-01-01 00:00:00.750", Resource="MicroVM B"),
])

fig = px.timeline(df, x_start="Start", x_end="Finish", y="Resource", color="Task")
fig.update_yaxes(autorange="reversed")
fig.update_xaxes(tickformat="%M:%S.%L")

pio.full_figure_for_development(fig, warn=False)

fig.write_image("assets/aws-lambda-slot-placement.pdf")

from time import sleep
sleep(1)

fig.write_image("assets/aws-lambda-slot-placement.pdf")
