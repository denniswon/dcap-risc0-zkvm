# DCAP Quote Verification using Risc0 zkVM Profiling

Profiling code for performing the Intel SGX / Intel TDX Quote Verification, including the host and guest program.

## Prerequisites

Install Rust and RISC Zero zkVM:

```bash
curl -L https://risczero.com/install | bash
```

Additionally, [install Go](https://go.dev/doc/install), which bundles with it the [pprof] tool.

### Running

Run the DCAP quote verification circuit profiling with:

```bash
RISC0_PPROF_OUT=./profile.pb RUST_LOG=info RISC0_DEV_MODE=1 RISC0_INFO=1 cargo run
```

The above command will run the DCAP quote verification circuit and write the profiling output to `profile.pb`.
Use the environment variable `RISC0_PPROF_OUT` to set the desired output path for the profiling data.

### Visualization

To visualize the profile using `pprof`, run:

```bash
go tool pprof -http=127.0.0.1:8000 profile.pb
```

Then navigate to [http://localhost:8000](http://localhost:8000) in your browser.

### Inline Functions

If you have compiled your program with debug symbols you can enable inline function tracking in the profiler by setting `RISC0_PPROF_ENABLE_INLINE_FUNCTIONS=yes` for more detailed profiles, but note that this will slow down the profiler.

### Executing the project locally in development mode

For faster iteration upon code changes, leverage [dev-mode].

```bash
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run
```

## Directory Structure

```text
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── bin
    │           └── main.rs
    └── src
        └── lib.rs
```

## Profiling Output

```bash
# RISC0_PPROF_OUT=./profile.pb RUST_LOG=info RISC0_DEV_MODE=1 RISC0_INFO=1 cargo run

ImageID: 70c5862009c0dc522d29dc7e0bfe181b41c2b8ad1a10fcb24b1d14b4dd07999c
WARNING: proving in dev mode. This will not generate valid, secure proofs.
2025-04-13T18:26:14.997940Z  INFO risc0_zkvm::host::server::exec::executor: execution time: 542.834583ms
2025-04-13T18:26:14.998158Z  INFO risc0_zkvm::host::server::session: number of segments: 6
2025-04-13T18:26:14.998180Z  INFO risc0_zkvm::host::server::session: 5767168 total cycles
2025-04-13T18:26:14.998182Z  INFO risc0_zkvm::host::server::session: 4990251 user cycles (86.53%)
2025-04-13T18:26:14.998185Z  INFO risc0_zkvm::host::server::session: 624576 paging cycles (10.83%)
2025-04-13T18:26:14.998186Z  INFO risc0_zkvm::host::server::session: 152341 reserved cycles (2.64%)
2025-04-13T18:26:14.998188Z  INFO risc0_zkvm::host::server::session: ecalls
2025-04-13T18:26:14.998190Z  INFO risc0_zkvm::host::server::session:  10616 BigInt calls, 1112128 cycles, (19.28%)
2025-04-13T18:26:14.998192Z  INFO risc0_zkvm::host::server::session:  38 Sha2 calls, 12672 cycles, (0.22%)
2025-04-13T18:26:14.998210Z  INFO risc0_zkvm::host::server::session:  62 Read calls, 1390 cycles, (0.02%)
2025-04-13T18:26:14.998215Z  INFO risc0_zkvm::host::server::session:  1 Terminate calls, 2 cycles, (0.00%)
2025-04-13T18:26:14.998217Z  INFO risc0_zkvm::host::server::session:  0 Write calls, 0 cycles, (0.00%)
2025-04-13T18:26:14.998218Z  INFO risc0_zkvm::host::server::session:  0 User calls, 0 cycles, (0.00%)
2025-04-13T18:26:14.998220Z  INFO risc0_zkvm::host::server::session:  0 Poseidon2 calls, 0 cycles, (0.00%)
2025-04-13T18:26:14.998222Z  INFO risc0_zkvm::host::server::session: syscalls
2025-04-13T18:26:14.998224Z  INFO risc0_zkvm::host::server::session:  26 Read calls
2025-04-13T18:26:14.998226Z  INFO risc0_zkvm::host::server::session:  1 Write calls
2025-04-13T18:26:14.998227Z  INFO risc0_zkvm::host::server::session:  0 VerifyIntegrity2 calls
2025-04-13T18:26:14.998229Z  INFO risc0_zkvm::host::server::session:  0 VerifyIntegrity calls
2025-04-13T18:26:14.998230Z  INFO risc0_zkvm::host::server::session:  0 ProveKeccak calls
2025-04-13T18:26:14.998232Z  INFO risc0_zkvm::host::server::session:  0 Keccak calls
Proving time: 645.320375ms
```
