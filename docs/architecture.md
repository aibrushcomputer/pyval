# Architecture

emailval is built with Rust at its core and exposed to Python via PyO3 bindings.

## Project Structure

```
emailval/
├── crates/
│   └── pyval-core/       # Rust core library
├── wrappers/
│   └── python/           # Python bindings
├── docs/                 # Documentation
└── test_data/            # Test fixtures
```

## Rust Core

The core validation logic is written in Rust for maximum performance.

### Key Optimizations

1. **Lookup Tables**: O(1) character validation using 256-byte tables
2. **SWAR**: SIMD Within A Register - processes 8 bytes at once using u64 bit manipulation
3. **Zero-Copy Validation**: `is_valid_ultra()` validates without heap allocations
4. **Branch Prediction**: Hot paths optimized for common cases

### Validation Flow

```
Input Email
    │
    ▼
Fast Path (ASCII-only)
    │
    ├──► Ultra-fast check (no allocation)
    │
    └──► Full Validation
            │
            ├──► Syntax Check
            │       ├──► Local part validation
            │       └──► Domain validation
            │
            ├──► IDN Processing (if needed)
            │
            └──► Normalization
```

## Python Bindings

PyO3 provides seamless Python integration:

- **Zero-cost abstraction**: Rust functions exposed directly
- **Type conversion**: Automatic PyObject ↔ Rust type conversion
- **Error handling**: Rust Results mapped to Python exceptions

## Performance Characteristics

| Operation | Time | Memory |
|-----------|------|--------|
| `is_valid_ultra()` | ~108 ns | Stack only |
| `is_valid()` | ~168 ns | Minimal heap |
| `validate_email()` | ~500 ns | Normalization heap |

The Python FFI overhead is approximately 108ns, which is the minimum time for any Python → Rust call.
