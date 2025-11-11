# Fuzzing

We recommend `cargo fuzz` for ergonomic fuzzing:

```bash
cargo install cargo-fuzz
cargo fuzz init
# Then add a target that feeds random bytes into Timecode::from_bytes_lossy
```

This repo also includes a minimal libfuzzer harness under `fuzz_targets/` if you prefer manual setup.
