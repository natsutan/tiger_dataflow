use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

//use onnx_if::onnx_if::ModelProto;
//use onnx_if::onnx_if::NodeProto;
//use onnx_if::ModelProto;
//use onnx_if::NodeProto;
use crate::onnx_if;

#[allow(dead_code)]
pub fn node_dump(model:&onnx_if::onnx::ModelProto) {

    let graph = model.get_graph();
    let nodes = graph.get_node();

    for node in nodes {
        println!("node: {:?}", node);
    }
}

pub fn write_dot(model:&onnx_if::onnx::ModelProto, filename:&str) {
    //ファイルを開く
    let fp = fs::File::create(filename).unwrap();
    let mut fb = BufWriter::new(fp);

    let graph = model.get_graph();
    let nodes = graph.get_node();

    let graph_name = get_model_name(model);
    fb.write(b"digraph ").unwrap();
    fb.write(graph_name.as_bytes()).unwrap();
    lb(&mut fb);

    //ノードの一覧を出力
    fb.write(b"// node define\n").unwrap();

    let mut name_gen = NodeNameGenerator{num :0, prefix :&"OP".to_string()};

    for node in nodes {
        let op_name = generate_name(&mut name_gen);
        let node_name = build_node_name(&node, op_name);
        fb.write(format!("{};\n", node_name).as_bytes()).unwrap();

        for input in node.get_input() {
            let src = input;
            let dst = node_name.clone();
            fb.write(format!("\"{}\" -> \"{}\";\n", src, dst).as_bytes()).unwrap();
        }

        for output in node.get_output() {
            let src = node_name.clone();
            let dst = output;
            fb.write(format!("\"{}\" -> \"{}\";\n", src, dst).as_bytes()).unwrap();
        }

    }

    rb(&mut fb);
}

fn get_model_name(model:&onnx_if::onnx::ModelProto) -> &str {
    model.get_graph().get_name()
}

//#[derive(Sized, Clone)]
struct NodeNameGenerator<'a> {
    num: u64,
    prefix: &'a String
}


fn generate_name(gen:&mut NodeNameGenerator) -> String{
    let name = format!("{}{}", gen.prefix, gen.num);
    gen.num = gen.num + 1;
    name
}

fn cr(fb:&mut BufWriter<File>) {
    fb.write(b"\n").unwrap();
}


fn lb(fb:&mut BufWriter<File>) {
    fb.write(b"{").unwrap();
    cr(fb);
}

fn rb(fb:&mut BufWriter<File>) {
    fb.write(b"}").unwrap();
    cr(fb);
}

fn build_node_name(node:&onnx_if::onnx::NodeProto, op_name:String) -> String {
    let op_type = node.get_op_type();
    format!("{}_{}", op_name, op_type)
}