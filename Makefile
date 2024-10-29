# Python commands
setup_python:
	pip install -r python_script/requirements.txt

run_python:
	python python_script/main.py

# Rust commands
setup_rust:
	cd rust_script && cargo build

run_rust:
	cd rust_script && cargo run --release

# Benchmarking commands
benchmark:
	time make run_python
	time make run_rust