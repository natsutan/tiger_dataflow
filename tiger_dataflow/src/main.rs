extern crate protobuf;

use std::fs::File;
use std::io::{BufReader};
use protobuf::{CodedInputStream, Message};

//protoc --rust_out . onnx_if.proto で生成されたonnx.rsを読み込む
mod onnx_if;
use onnx_if::onnx::ModelProto;

fn main() {
    let file = File::open("../data/resnet50/model.onnx").expect("fail to open file");
    let mut buffered_reader = BufReader::new(file);
    let mut cis = CodedInputStream::from_buffered_reader(&mut buffered_reader);

    let mut u = ModelProto::new();
    u.merge_from(&mut cis).expect("fail to merge");

    onnx_if::onnx_graph::write_dot(&u, "../output/sq.dot");


    println!("producer name: {}", u.get_producer_name());
}
