import matplotlib as mpl
import pandas as pd
import seaborn as sns
import fitter as fitter
import numpy as np
from scipy.stats import weibull_min

col_width = 2.96289

file_path = 'tpch-q2-resp-times.json'
df = pd.read_json(file_path)

mpl.use("pgf")

import matplotlib.pyplot as plt
sns.set_theme()

fig, ax = plt.subplots(figsize=(col_width * 0.8, col_width * 0.8))

df['duration'].plot(kind='hist', bins=35, edgecolor='black', ax=ax, density=True, label="TCP-H Query 2")

duration = df['duration'].values
f = fitter.Fitter(
    duration,
    distributions=['weibull_min', 'norm', 'beta', 'betaprime']
)
f.fit()

shape, thresh, scale = f.fitted_param["weibull_min"]

print(f"W({shape}, {thresh}, {scale})")

# Generate values for the Weibull distribution
x = np.linspace(100, max(duration) + 100, 200)
weibull_pdf = weibull_min.pdf(x, shape, thresh, scale)

# Plot the Weibull distribution on the same axes
ax.plot(x, weibull_pdf, label='Weibull Distribution', color='red')

# Add a legend
ax.legend(loc='lower center', bbox_to_anchor=(0.5, -0.7))

plt.xlabel('Response Time (ms)')
plt.ylabel('Frequency')
plt.title('TPC-H query 2 response time\ndistribution from AWS Lambda')
plt.savefig("assets/tpch-q2-resp-times.pgf", bbox_inches="tight")
