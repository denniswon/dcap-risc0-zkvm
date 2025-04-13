use risc0_build::embed_methods_with_options;
use risc0_build::{DockerOptionsBuilder, GuestOptionsBuilder};
use std::collections::HashMap;

fn main() {
    let docker_options = DockerOptionsBuilder::default()
        .root_dir("../")
        .env(vec![("ENV_VAR".to_string(), "value".to_string())])
        .build()
        .unwrap();

    let guest_options = GuestOptionsBuilder::default()
        .use_docker(docker_options)
        .build()
        .unwrap();

    // Generate Rust source files for the methods crate.
    embed_methods_with_options(HashMap::from([("dcap_guest", guest_options)]));
}
