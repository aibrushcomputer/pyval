#!/usr/bin/env python3
"""Baseline benchmarks for python-email-validator."""
import time
import statistics
import json
from pathlib import Path

# Reference implementation
from email_validator import validate_email as py_validate_email, EmailNotValidError

# Our test data
import sys
sys.path.insert(0, '/home/aibrush/pyval/test_data')
from emails import VALID_EMAILS, INVALID_EMAILS, generate_bulk_emails

RESULTS_FILE = Path("/home/aibrush/pyval/baseline_results.json")
ITERATIONS = 1000
BULK_SIZE = 10000

def benchmark(func, iterations=ITERATIONS):
    """Run function multiple times and return stats."""
    times = []
    for _ in range(iterations):
        start = time.perf_counter_ns()
        func()
        end = time.perf_counter_ns()
        times.append(end - start)
    return {
        "mean_ns": statistics.mean(times),
        "median_ns": statistics.median(times),
        "min_ns": min(times),
        "max_ns": max(times),
        "stdev_ns": statistics.stdev(times) if len(times) > 1 else 0,
        "iterations": iterations
    }

def run_benchmarks():
    results = {}
    
    # Single email validation
    test_email = "user.name+tag@example.com"
    print(f"Benchmarking single email: {test_email}")
    results["single_valid"] = benchmark(
        lambda: py_validate_email(test_email, check_deliverability=False)
    )
    
    # Invalid email (should be fast to reject)
    invalid_email = "invalid@@email"
    print(f"Benchmarking invalid email: {invalid_email}")
    def validate_invalid():
        try:
            py_validate_email(invalid_email, check_deliverability=False)
        except EmailNotValidError:
            pass
    results["single_invalid"] = benchmark(validate_invalid)
    
    # Batch validation
    bulk_emails = generate_bulk_emails(BULK_SIZE)
    print(f"Benchmarking batch of {BULK_SIZE} emails...")
    
    def validate_batch():
        for email in bulk_emails[:100]:  # 100 per iteration
            try:
                py_validate_email(email, check_deliverability=False)
            except EmailNotValidError:
                pass
    
    results["batch_100"] = benchmark(validate_batch, iterations=100)
    
    # Internationalized email
    idn_email = "用户@例子.广告"
    print(f"Benchmarking IDN email: {idn_email}")
    results["idn_email"] = benchmark(
        lambda: py_validate_email(idn_email, check_deliverability=False)
    )
    
    # With normalization
    print("Benchmarking with normalization...")
    results["with_normalization"] = benchmark(
        lambda: py_validate_email("User.Name@EXAMPLE.COM", check_deliverability=False)
    )
    
    # Save results
    with open(RESULTS_FILE, "w") as f:
        json.dump(results, f, indent=2)
    
    print(f"\nBaseline results saved to {RESULTS_FILE}")
    print("\nSummary (mean times):")
    for name, stats in results.items():
        print(f"  {name}: {stats['mean_ns']/1000:.2f} µs")
    
    return results

if __name__ == "__main__":
    run_benchmarks()
