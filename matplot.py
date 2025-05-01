import matplotlib.pyplot as plt
import numpy as np
from collections import defaultdict

# Configuration - Adjust these values
TIME_BINNING_TOLERANCE = 10.0  # ms (points within this range will be grouped)
ROUNDING_DECIMALS = 1  # For initial time grouping

# Data loading
data = defaultdict(lambda: {'times': [], 'scores': []})
with open('data.txt', 'r') as f:
    for line in f:
        parts = line.strip().split()
        if len(parts) >= 4:  # Format: Algorithm RunID Time Score
            algo = parts[0]
            data[algo]['times'].append(float(parts[2]))
            data[algo]['scores'].append(float(parts[3]))

# Setup plot
plt.figure(figsize=(12, 6))
plt.xlabel('Time (ms)')
plt.ylabel('Score')
plt.ylim(-750000.0, 100000)
plt.xlim(0, 3000)
plt.title(f'Algorithm Performance (Time bins: ±{TIME_BINNING_TOLERANCE}ms)')
plt.grid(True, alpha=0.3)

for i, (algo, values) in enumerate(data.items()):
    times = np.array(values['times'])
    scores = np.array(values['scores'])
    
    # Initial coarse grouping (improves efficiency)
    rounded_times = np.round(times, decimals=ROUNDING_DECIMALS)
    unique_rounded = np.unique(rounded_times)
    
    # Fine-grained binning with tolerance
    final_bins = []
    for t in unique_rounded:
        # Find all times within tolerance of this rounded time
        mask = np.abs(times - t) <= TIME_BINNING_TOLERANCE
        if np.any(mask):
            final_bins.append({
                'time': np.mean(times[mask]),  # Center of the bin
                'scores': scores[mask]
            })
    
    # Calculate statistics
    bin_times = [b['time'] for b in final_bins]
    means = [np.mean(b['scores']) for b in final_bins]
    mins = [np.min(b['scores']) for b in final_bins]
    maxs = [np.max(b['scores']) for b in final_bins]
    
    # Plotting
    color = plt.cm.tab10(i)
    plt.plot(bin_times, means, color=color, linewidth=2.5, label=algo)
    plt.fill_between(bin_times, mins, maxs, color=color, alpha=0.15)

plt.legend(bbox_to_anchor=(1.05, 1))
plt.tight_layout()
plt.show()
