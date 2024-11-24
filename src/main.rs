use anyhow::anyhow;
use lunar::{compute::Runner, model::ComputationalModel, utils};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    }
}

fn run() -> anyhow::Result<()> {
    let arg = std::env::args()
        .nth(1)
        .ok_or(anyhow!("no input file provided"))?;

    let text = std::fs::read_to_string(&arg).map_err(|e| anyhow!("reading from input: {e}"))?;

    let args: Vec<String> = std::env::args().skip(2).collect();

    let model: ComputationalModel =
        toml::from_str(&text).map_err(|e| anyhow!("deserializing model: {e}"))?;

    let (inputs, outputs) = utils::parse_args(args, &model)?;

    let mut runner = Runner::new();

    let results = runner.run(model, inputs, outputs)?;

    for (name, value) in results {
        println!("{name}: {value:?}");
    }

    Ok(())
}
