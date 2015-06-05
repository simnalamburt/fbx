use std::fs::File;
use std::io::Read;
use std::io::BufReader;

extern crate fbx;
use fbx::parser::ascii::*;

fn main() {
    let fixture = "fixtures/2013_ASCII/Cinema4D.fbx";
    let mut source = String::new();
    let mut fbx_file_reader = BufReader::new(File::open(fixture).unwrap());
    fbx_file_reader.read_to_string(&mut source);
    let tokens = tokenize(&source);
    println!("{}\n", source);
    for token in tokens.iter() {
        println!("{}\t{}", match token.token_type {
            TokenType::OpenBracket => "{",
            TokenType::CloseBracket => "}",
            TokenType::Data => "DATA",
            TokenType::Comma => ",",
            TokenType::Key => "KEY",
        }, token.contents);
    }
}
