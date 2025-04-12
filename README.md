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
