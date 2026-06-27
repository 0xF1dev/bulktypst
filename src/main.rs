mod csv_parser;
mod typst_renderer;

use clap::Parser;
use colored::Colorize;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Typst template")]
    template: String,

    #[arg(short, long, help = "CSV file to get the data from")]
    csv: String,

    #[arg(short, long, help = "Output directory")]
    output: String,
}

fn main() {
    let args = Args::parse();

    let data = csv_parser::get_data(args.csv).expect(&*"could not parse the csv file.".red());

    for (i, entry) in data.iter().enumerate() {
        let output = typst_renderer::render(args.template.clone(), entry.clone())
            .expect(&*"could not render pdf".red());
        fs::create_dir_all(args.output.clone()).expect(&*"could not create directory".red());
        fs::write(format!("{}/{}.pdf", args.output.clone(), i), output)
            .expect(&*"could not write output file".red());
        println!("{} {i}", "written file".green())
    }

    println!("{}", "✓ completed".green().bold())
}
