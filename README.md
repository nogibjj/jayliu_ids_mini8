# JAYLIU_IDS_MINI8 - Python to Rust Data Processing Project

This project involves rewriting a Python data processing script in Rust to highlight improvements in speed and resource usage.

## Structure
- `python_script/`: Contains the Python version.
- `rust_script/`: Contains the Rust version.
- `data/`: Contains input and output data files.

## Setup
Install dependencies for Python and Rust by running:
```bash
make setup_python
make setup_rust
```
# Performance Comparison Report

## Functionality in Rust
The Rust script successfully replicates the functionality of the Python script, including reading a dataset, filtering rows where `year > 2000`, grouping by `month` and `day_of_week`, and writing the results to CSV files.

## Speed and Resource Usage Improvements
Benchmark results show that the Rust script is significantly faster and uses fewer CPU resources compared to the Python version, as summarized below:

| Metric       | Python Script | Rust Script | Improvement      |
|--------------|---------------|-------------|------------------|
| Real Time    | 0.65 s        | 0.45 s      | ~30.8% faster    |
| User Time    | 1.70 s        | 0.06 s      | ~97% less usage  |
| Sys Time     | 0.63 s        | 0.04 s      | ~93% less usage  |

## Conclusion
The Rust script provides a substantial performance boost, particularly in CPU usage, making it a more efficient choice for large-scale data processing tasks.

