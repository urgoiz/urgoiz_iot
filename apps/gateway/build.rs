fn main() {
    prost_build::compile_protos(&["proto/sensor.proto"], &["proto/"]).unwrap();
}