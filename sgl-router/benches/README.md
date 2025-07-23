# SGLang Router Load Testing & Benchmarks

This directory contains all performance testing tools for the SGLang router, including load testing utilities and comprehensive benchmarks.

## Directory Structure

```
benches/
├── load_test.rs               # CLI load testing tool (bin target)
├── streaming_load_test.rs     # Criterion benchmark for streaming
├── policy_comparison.rs       # Criterion benchmark for policies
├── request_processing.rs      # Criterion benchmark for request processing
└── README.md                  # This documentation
```

## Overview

The SGLang router provides two main approaches for load testing:

1. **Standalone Load Test Tool** (`cargo run --bin load_test`) - Interactive CLI tool for quick testing
2. **Benchmark Suite** (`cargo bench`) - Comprehensive benchmarks using the Criterion framework

## Standalone Load Test Tool

The standalone load test tool provides a user-friendly interface for running load tests with real-time progress tracking and detailed performance reports.

### Basic Usage

```bash
# Default configuration (1000 requests, 4 workers, batch size 100)
cargo run --release --bin load_test

# Custom configuration
cargo run --release --bin load_test -- <requests> <workers> <batch_size>

# Examples
cargo run --release --bin load_test -- 5000 8 200
cargo run --release --bin load_test -- 10000 10 500
```

### Command Line Options

```bash
cargo run --release --bin load_test -- [OPTIONS] <requests> <workers> <batch_size>

Arguments:
  <requests>    Number of requests to send (default: 1000)
  <workers>     Number of mock workers to spawn (default: 4)
  <batch_size>  Number of concurrent requests per batch (default: 100)

Options:
  -d, --delay <delay>          Worker response delay in milliseconds (default: 0)
  -p, --port <port>            Router port to use (default: 3011)
  -e, --endpoint <name>        Endpoint to test: generate, chat, completions (default: generate)
  --no-stream                  Test non-streaming requests (default: streaming)
  -r, --routing-mode <mode>    Routing mode: regular, pd (default: regular)
  --policy <policy>            Load balancing policy: random, round_robin, power_of_two, cache_aware (default: random)
  --prefill-workers <num>      Number of prefill workers for PD mode (default: 2)
  --decode-workers <num>       Number of decode workers for PD mode (default: 2)
  -h, --help                   Print help information
```

### Endpoints

The tool supports testing three different endpoints:

1. **generate** → `/generate` (SGLang native endpoint)
2. **chat** → `/v1/chat/completions` (OpenAI Chat API compatible)
3. **completions** → `/v1/completions` (OpenAI Completions API compatible)


### Example Output

```
=== SGLang Router Load Test ===
Configuration:
  Requests: 10000
  Workers: 10
  Batch size: 200
  Worker delay: 0ms
  Mode: Streaming
  Endpoint: /generate
  Routing mode: regular
  Policy: random

Starting 10 mock workers...
  Worker 1 (Regular) started at http://127.0.0.1:30000
  ...
  Worker 10 (Regular) started at http://127.0.0.1:30009

[██████████████████████████████████████████████████] 100.0% | 10000 / 10000 | 9476 req/s | 0 failed

================================================================================
                          LOAD TEST RESULTS
================================================================================

📊 Summary:
  Total requests:       10000
  Completed:           10000 (100.0%)
  Successful:          10000 (100.0%)
  Failed:              0 (0.0%)

⏱️  Performance:
  Total time:          1.06s
  Throughput:          9476 requests/second
  Average latency:     8.45ms
  Estimated P50:       6.76ms
  Estimated P95:       12.68ms
  Estimated P99:       16.90ms

🔄 Worker Distribution:
  Worker 1: 1000 requests (10.0%)
  Worker 2: 1000 requests (10.0%)
  ...
  Worker 10: 1000 requests (10.0%)

⚙️  Configuration:
  Workers:             10
  Batch size:          200
  Worker delay:        0ms
  Router port:         3011
  Routing mode:        regular
  Policy:              random

================================================================================
```

## Benchmark Suite

The benchmark suite provides more rigorous performance testing using the Criterion framework.

### Available Benchmarks

#### 1. Request Processing Benchmark
Tests the raw request processing performance of the router.

```bash
cargo bench request_processing
```

#### 2. Streaming Load Test Benchmark
Comprehensive benchmark for streaming request handling with various configurations.

```bash
cargo bench streaming_load_test
```

This benchmark includes:
- **Throughput tests**: Measures requests/second with different loads (100, 500, 1000 requests)
- **Response parsing overhead**: Compares performance with and without SSE parsing
- **Worker scaling**: Tests performance with 1, 2, 4, and 8 workers

#### 3. Policy Comparison Benchmark
Tests performance of different load balancing policies.

```bash
cargo bench policy_comparison
```

### Running Specific Benchmarks

To run a specific benchmark group:
```bash
cargo bench streaming_throughput
cargo bench response_parsing_overhead
cargo bench worker_scaling
```

### Benchmark Output

Criterion generates detailed HTML reports in `target/criterion/` with:
- Performance graphs
- Statistical analysis
- Comparison with previous runs
- Regression detection

## Testing Scenarios

### 1. Basic Performance Test
Test the router's baseline performance:
```bash
cargo run --release --bin load_test -- 1000 4 100
```

### 2. High Concurrency Test
Test with many concurrent requests:
```bash
cargo run --release --bin load_test -- 10000 10 1000
```

### 3. Latency Under Load
Test with simulated worker delays:
```bash
cargo run --release --bin load_test -- 5000 8 100 --delay 10
```

### 4. Worker Scaling Test
Test how performance scales with workers:
```bash
# 2 workers
cargo run --release --bin load_test -- 5000 2 100

# 4 workers
cargo run --release --bin load_test -- 5000 4 100

# 8 workers
cargo run --release --bin load_test -- 5000 8 100
```

### 5. Streaming vs Non-Streaming Comparison
Compare performance between streaming and non-streaming:
```bash
# Streaming (default)
cargo run --release --bin load_test -- 1000 4 100

# Non-streaming
cargo run --release --bin load_test -- 1000 4 100 --no-stream
```

### 6. Different Endpoint Testing
Test various API endpoints:
```bash
# Native SGLang endpoint
cargo run --release --bin load_test -- 1000 4 100 -e generate

# OpenAI Chat API
cargo run --release --bin load_test -- 1000 4 100 -e chat

# OpenAI Completions API
cargo run --release --bin load_test -- 1000 4 100 -e completions --no-stream
```

### 7. Policy Comparison
Test different load balancing policies:
```bash
# Random policy (default)
cargo run --release --bin load_test -- 5000 6 200 --policy random

# Round-robin policy
cargo run --release --bin load_test -- 5000 6 200 --policy round_robin

# Power-of-two choices
cargo run --release --bin load_test -- 5000 6 200 --policy power_of_two

# Cache-aware routing
cargo run --release --bin load_test -- 5000 6 200 --policy cache_aware
```

### 8. PD Mode Testing
Test with prefill and decode workers:
```bash
# PD mode with 3 prefill and 5 decode workers
cargo run --release --bin load_test -- 5000 0 200 --routing-mode pd --prefill-workers 3 --decode-workers 5
```

## Performance Optimization Tips

1. **Always use release mode** for accurate results
2. **Close unnecessary applications** to reduce system noise
3. **Run multiple times** and look at median values
4. **Monitor system resources** during tests
5. **Test on dedicated hardware** when possible

## Interpreting Results

### Key Metrics

1. **Throughput (req/s)**: Higher is better
   - Good: > 5,000 req/s
   - Excellent: > 10,000 req/s

2. **Latency**: Lower is better
   - P50: Median latency
   - P95: 95% of requests complete within this time
   - P99: 99% of requests complete within this time

3. **Success Rate**: Should be close to 100%
   - < 99%: Investigate errors
   - < 95%: Serious issues

4. **Worker Distribution**: Should be relatively even
   - Large imbalances indicate routing issues

## Troubleshooting

### Common Issues

1. **"Address already in use" errors**
   - Previous test didn't clean up properly
   - Solution: Wait a moment or use different ports

2. **Worker cleanup hanging**
   - Mock workers not shutting down gracefully
   - Solution: The tool now has 5-second timeouts per worker

3. **Low throughput**
   - Check worker delays are set to 0 for max performance tests
   - Ensure release mode is used
   - Check system resources

4. **Uneven worker distribution**
   - May indicate issues with the routing policy
   - Check router configuration

## Advanced Usage

### Custom Mock Worker Behavior

The mock workers can be configured with:
- Response delays
- Failure rates
- Different worker types (Regular, Prefill, Decode)
- Health status simulation

### Integration with CI/CD

The load test tool can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run Load Test
  run: |
    cargo build --release --bin load_test
    cargo run --release --bin load_test -- 5000 8 200
```

### Comparing Results

Track performance over time by saving results:

```bash
# Run test and save output
cargo run --release --bin load_test -- 10000 10 200 > results_$(date +%Y%m%d_%H%M%S).txt

# Compare results
diff results_20240115_120000.txt results_20240116_120000.txt
```

## Benchmark Development

To add new benchmarks:

1. Create a new `.rs` file in `benches/`
2. Import common utilities: `mod common;`
3. Add to `Cargo.toml` if it's a criterion benchmark:
   ```toml
   [[bench]]
   name = "your_benchmark"
   harness = false
   path = "benches/your_benchmark.rs"
   ```
4. Register benchmark functions in the `criterion_group!` macro
5. Document the benchmark purpose and parameters
