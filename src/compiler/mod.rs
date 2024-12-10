use std::process::Command;

pub fn compile(source: String, libraries: Vec<String>) -> anyhow::Result<()> {
    std::fs::write("main.c", source)?;

    Command::new("cc")
        .args(["-O2", "-c", "main.c"])
        .spawn()?
        .wait()?;

    let mut command = Command::new("cc");
    command.args(["-shared", "-o", "libmain.so", "main.o", "-L."]);

    for lib in libraries {
        command.arg(format!("-l{lib}"));
    }

    command.spawn()?.wait()?;

    Ok(())
}
