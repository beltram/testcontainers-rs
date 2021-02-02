use testcontainers::{images::hello_world::HelloWorld, *};

#[test]
fn podman_can_run_hello_world() {
    let podman = clients::Cli::podman();

    let _container = podman.run(HelloWorld);
}
