fn main() {
    tonic_build::compile_protos("proto/proving_network.proto").unwrap();
}
