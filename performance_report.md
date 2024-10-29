# Performance Comparison Report

## 1. Overview
This report compares the performance of a data processing script implemented in Python and Rust. The primary operations include loading a dataset, filtering rows where `year > 2000`, grouping by `month` and `day_of_week`, and saving the results to CSV files.

The Rust implementation is expected to demonstrate improvements in execution speed and resource usage, thanks to Rust's efficiency in managing system resources.

## 2. Functionality
The Rust script successfully replicates the functionality of the Python script. Key operations include:
- Loading data from `US_birth.csv`
- Filtering records where `year > 2000`
- Grouping by `month` and `day_of_week` and calculating total `births`
- Saving the summary data to `birth_summary_by_month.csv` and `birth_summary_by_day.csv`

Both scripts were verified to produce identical results, ensuring functional equivalence.

## 3. Benchmark Results
The benchmark results below compare the execution times of the Python and Rust versions, run on the same dataset.

| Metric       | Python Script | Rust Script | Improvement      |
|--------------|---------------|-------------|------------------|
| Real Time    | 0.65 s        | 0.45 s      | ~30.8% faster    |
| User Time    | 1.70 s        | 0.06 s      | ~97% less usage  |
| Sys Time     | 0.63 s        | 0.04 s      | ~93% less usage  |

### Explanation of Metrics
- **Real Time**: Total wall-clock time taken to execute each script.
- **User Time**: CPU time spent in user-mode (CPU usage in application code).
- **Sys Time**: CPU time spent in kernel-mode (e.g., file I/O operations).

The Rust script completes faster and uses significantly less CPU time compared to the Python script, indicating improved efficiency.

## 4. Analysis
The Rust implementation offers substantial performance gains over the Python version, particularly in:
- **Execution Speed**: Rust’s real-time performance is around 30% faster than Python.
- **CPU Efficiency**: Rust uses 97% less CPU in user mode, showcasing its resource efficiency for CPU-bound tasks.
- **System Resource Usage**: Rust also uses less system time, indicating faster file I/O handling.

## 5. Conclusion
The Rust implementation of the data processing script is both faster and more resource-efficient than the Python version, making it a better choice for large-scale data processing tasks.
