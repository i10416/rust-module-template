fn main() -> Result<(),std::io::Error> {
     tonic_build::configure()
       .out_dir("src/gen")
       .build_server(true)
       .compile(
        &["proto/hello.proto"],
        &["proto"]
       )
}