use clap::{Parser, ValueEnum};

mod days;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: usize,

    #[arg(short, long, value_enum)]
    part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Part {
    #[clap(alias = "1")]
    One,
    #[clap(alias = "2")]
    Two,
}

fn main() {
    let args = Args::parse();

    if let Some(day) = days::Days::from_day_number(args.day) {
        match args.part {
            Part::One => println!("{}", day.part_one()),
            Part::Two => println!("{}", day.part_two()),
        };
    } else {
        eprintln!("Day {} not implemented", args.day);
    }
}
