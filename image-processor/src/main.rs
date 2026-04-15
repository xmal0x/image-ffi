use clap::Parser;

#[derive(Parser)]
#[command(name = "image-processor", about = "Image processor cli", version)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(long)]
    plugin: String,
    #[arg(long)]
    params: String,
    #[arg(long)]
    plugin_path: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let Cli {
        input,
        output,
        plugin,
        params,
        plugin_path,
    } = cli;

    let plugin_path = plugin_path.unwrap_or("target/debug".into());

    println!(
        "Params: {} {} {} {} {}",
        input, output, plugin, params, plugin_path
    );
}
