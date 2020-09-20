extern crate protoc_rust;
use protoc_rust::Customize;

/// compile proto file to rust file
/// Please run the proto compiler in Linux
fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "../pbdemo/src/protos",
        input: &["pbdemo/Person.proto", "pbdemo/Fruit.proto"],
        includes: &["./pbdemo"],
        customize: Customize {
            ..Default::default()
        },
    })
    .expect("protoc");
}
