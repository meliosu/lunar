use std::{
    io::Write,
    process::{Command, Stdio},
};

use clap::Parser;

use lunar::{args::Args, compiler, lexer, parser, runtime, translator};

fn main() {
    let args = Args::parse();
    let text = std::fs::read_to_string(args.input).unwrap();

    let tokens = lexer::lex(&text).unwrap();
    eprintln!("{tokens:#?}");

    for (i, token) in tokens.iter().enumerate() {
        eprintln!("{i}: {:#?}", token)
    }

    let ast = parser::parse(tokens).unwrap();
    eprintln!("{ast:#?}");

    let code = translator::translate(ast).unwrap();
    println!("{code}");

    let mut command = Command::new("clang-format")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    command
        .stdin
        .take()
        .unwrap()
        .write(code.as_bytes())
        .unwrap();

    command.wait().unwrap();

    //compiler::compile(code, args.link.clone()).unwrap();
    //
    //runtime::launch("libmain.so".into(), args.link.clone()).unwrap();
}
