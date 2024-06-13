from datetime import datetime, timedelta, timezone

from cost_breakdown import generate_cost_breakdown
from cost_breakdown_olap import generate_cost_breakdown_olap
from continuation_breakdown import generate_continuation_breakdown
from estimation_accuracy import generate_estimation_accuracy

tz = timezone(offset=timedelta(hours=1))

time_start = datetime(year=2024, month=6, day=13, hour=12, minute=16, second=29, tzinfo=tz)
time_end = datetime(year=2024, month=6, day=13, hour=12, minute=27, second=24, tzinfo=tz)

from pandas import option_context
with option_context('display.max_rows', None, 'display.max_columns', None):
    generate_cost_breakdown(time_start, time_end)
    generate_cost_breakdown_olap(time_start, time_end)
    generate_continuation_breakdown(time_start, time_end)
    generate_estimation_accuracy(time_start, time_end)
