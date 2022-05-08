fn main() {
    let files = [
        "proto/messages.proto",
        "proto/users.proto",
        "proto/products.proto",
    ];

    for file in files {
        println!("cargo:rerun-if-changed={}", file);
        tonic_build::compile_protos(file)
            .unwrap_or_else(|e| panic!("Error compiling {}:\n{}", file, e));
    }
}
