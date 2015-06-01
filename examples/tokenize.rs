extern crate fbx;

fn main() {
    use fbx::parser::ascii::tokenize;
    let _ = tokenize("abcd");
}
