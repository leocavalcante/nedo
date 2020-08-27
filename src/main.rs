use std::fs::File;

use html5ever::{parse_document, ParseOpts};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::{TreeBuilderOpts};
use markup5ever::{local_name};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

fn output(node: &Handle) {
    match node.children.borrow().first().unwrap().data {
        NodeData::Text {ref contents} => {
            println!("{}", contents.borrow().to_string())
        },
        _ => {}
    }
}

fn eval(node: &Handle) -> Result<(), String> {
    match node.data {
        NodeData::Element { ref name, .. } => {
            match name.local {
                local_name!("output") => output(node),
                _ => {}
            }
        }
        _ => {}
    }

    node.children.borrow().iter().for_each(|node| {
        eval(node);
    });
    Ok(())
}

fn parse() -> Result<(), String> {
    let mut args = std::env::args();
    let filename = args.nth(1).ok_or("missing filename")?;
    let mut file = File::open(filename).map_err(|err| err.to_string())?;

    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut file).map_err(|err| err.to_string())?;

    return eval(&dom.document);
}

fn main() {
    if let Err(e) = parse() {
        eprintln!("Error: {}", e);
    }
}
