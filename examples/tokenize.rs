extern crate fbx;

fn main() {
    use fbx::parser::ascii::*;
    let tokens = tokenize("FBXHeaderExtension:  { FBXHeaderVersion: 1003");
    for token in tokens.iter() {
        println!("{}", match token.token_type {
            TokenType::OpenBracket => "{",
            TokenType::CloseBracket => "}",
            TokenType::Data => "DATA",
            TokenType::Comma => ",",
            TokenType::Key => "KEY",
        });
    }
}
