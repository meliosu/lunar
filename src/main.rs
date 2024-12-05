use lunar::{lexer, parser, translator};

fn main() {
    let text = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let tokens = lexer::lex(&text).unwrap();
    eprintln!("{tokens:#?}");

    for (i, token) in tokens.iter().enumerate() {
        eprintln!("{i}: {:#?}", token)
    }

    let ast = parser::parse(tokens).unwrap();
    eprintln!("{ast:#?}");

    let code = translator::translate(ast).unwrap();
    println!("{code}");
}
