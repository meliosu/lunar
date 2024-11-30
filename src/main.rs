use anyhow::bail;
use lunar::{lexer, parser, translator};

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
    }
}

fn run() -> anyhow::Result<()> {
    let Some(source) = std::env::args().nth(1) else {
        bail!("no input file");
    };

    let source = std::fs::read_to_string(source)?;

    let tokens = lexer::lex(&source)?;

    for (i, token) in tokens.iter().enumerate() {
        eprintln!("{i}: {token:#?}");
    }

    eprintln!("{tokens:#?}");

    let ast = parser::parse(tokens)?;
    eprintln!("{ast:#?}");

    let code = translator::translate(ast)?;
    println!("{code}");

    Ok(())
}
