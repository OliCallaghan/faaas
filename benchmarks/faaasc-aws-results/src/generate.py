from datetime import datetime, timezone

from cost_breakdown import generate_cost_breakdown
from continuation_breakdown import generate_continuation_breakdown
from estimation_accuracy import generate_estimation_accuracy

time_experiment = datetime(year=2024, month=6, day=11, hour=18, minute=40, second=0, tzinfo=timezone.utc)

from pandas import option_context
with option_context('display.max_rows', None, 'display.max_columns', None):
    # generate_cost_breakdown(time_experiment)
    # generate_continuation_breakdown(time_experiment)
    generate_estimation_accuracy(time_experiment)
