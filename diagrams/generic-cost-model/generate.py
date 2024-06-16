import seaborn as sns
import matplotlib.pyplot as plt
import numpy as np

C_r = 0.0000166667 * (128 / 1024)
t_warmup = 0.003

def compute_splitting_threshold(x):
    return (C_r * t_warmup + x) / C_r

invoc_cost = np.linspace(0, 0.0000002, 400)
durtn_cost = compute_splitting_threshold(invoc_cost)

# Create a DataFrame for seaborn
import pandas as pd
data = pd.DataFrame({'invoc_cost': invoc_cost, 'durtn_cost': durtn_cost})

print(data["durtn_cost"])

# Plot using seaborn
sns.set(style="whitegrid")
sns \
    .lineplot(x='invoc_cost', y='durtn_cost', data=data, label='Profitability threshold for code splitting') \
    .legend(loc='lower center', bbox_to_anchor=(0.5, -0.75))

plt.axhline(y=0.0028898, color='r', linestyle='--', label='Average petstore SQL query duration')

plt.xlim(0, 2e-7)
plt.xlabel('Invocation cost (USD)')
plt.ylabel('Response time of asynchronous service (sec)')
plt.title('Profitability threshold for code splitting when varying invocation cost')
plt.show()
