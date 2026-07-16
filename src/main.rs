use clap::Parser;
mod cli;
mod core;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    gui: bool,
}

fn main() {
    let args = Args::parse();

    if args.gui {
        // TODO: Launch egui GUI
        println!("GUI mode not implemented yet.");
    } else {
        cli::run();
    }
}
