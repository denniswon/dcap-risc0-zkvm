use risc0_build::{embed_methods_with_options, DockerOptions, GuestOptions};
use std::collections::HashMap;

fn main() {
    // Generate Rust source files for the methods crate.
    embed_methods_with_options(HashMap::from([(
        "dcap_guest",
        GuestOptions {
            features: Vec::new(),
            use_docker: Some(DockerOptions {
                root_dir: Some("../".into())
            }),
        },
    )]));

}