use std::io::Result;

fn main() -> Result<()> {
    protobuf_codegen::Codegen::new()
        .protoc()
        .includes(["../schemas"])
        .input("../schemas/message.proto")
        .cargo_out_dir("protobuf")
        .run_from_script();

    prost_build::compile_protos(&["../schemas/message.proto"], &["../schemas"])?;

    ::capnpc::CompilerCommand::new()
        .src_prefix("../schemas")
        .file("../schemas/message.capnp")
        // .output_path("capnps")
        .run()
        .expect("capnp compilation failed");

    Ok(())
}
