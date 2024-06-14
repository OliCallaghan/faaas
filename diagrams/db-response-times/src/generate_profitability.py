import matplotlib as mpl
import pandas as pd
import seaborn as sns
import fitter as fitter
import numpy as np
from scipy.stats import weibull_min

col_width = 2.96289

mpl.use("pgf")
mpl.rcParams.update({
    "pgf.texsystem": "pdflatex",
    'font.family': 'serif',
    'font.size' : 8,
    'text.usetex': True,
    'pgf.rcfonts': False,
})

import matplotlib.pyplot as plt
sns.set_theme()

fig, ax = plt.subplots(figsize=(col_width * 0.8, col_width * 0.8))

shape, thresh, scale = 2.4, 75, 50

print(f"W({shape}, {thresh}, {scale})")

# Generate values for the Weibull distribution
x = np.linspace(0, 300, 300)
weibull_pdf = weibull_min.pdf(x, shape, thresh, scale)

# Plot the Weibull distribution on the same axes
ax.plot(x, weibull_pdf, label='Async request to split around', color='red')

min_yield_threshold = 96

ax.axvline(x=min_yield_threshold, color='green', linestyle='--', label='\^t (profitability threshold)')
# Fill the area under the Weibull PDF from x=96 to the right off to 300 in translucent blue

# Compute the area under the curve to the right of x = min_yield_threshold
prob_profit = weibull_min.sf(min_yield_threshold, shape, thresh, scale)
print(f"Area to the right of x = {min_yield_threshold}: {prob_profit}")

p_profit = "\\textrm{P}(\\textrm{profit})"
p_not_profit = "\\textrm{P}(\\neg\\textrm{profit})"
ax.fill_between(x, weibull_pdf, where=(x >= min_yield_threshold), color='green', alpha=0.3, label=f"${p_profit} \\approx {prob_profit.round(2)}$")
ax.fill_between(x, weibull_pdf, where=(x <= min_yield_threshold), color='red', alpha=0.3, label=f"${p_not_profit}\\approx {(1 - prob_profit).round(2)}$")

# Add a legend
ax.legend(loc='lower center', bbox_to_anchor=(0.5, -0.9))

plt.xlabel('Response Time (ms)')
plt.ylabel('Frequency')
plt.title('Profitability of splitting')
plt.savefig("assets/split-profitability.pgf", bbox_inches="tight")
