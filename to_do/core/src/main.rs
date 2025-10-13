mod enums;
mod structs;
mod api;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// first name of user
    #[arg(short, long)]
    first_name: String,
    /// last name of user
    #[arg(short, long)]
    last_name: String,
    /// age of user
    #[arg(short, long, default_value_t = 1)]
    age: u8,
}

fn main() {
   let args = Args::parse();
   println!("{:?}", args.first_name);
   println!("{:?}", args.last_name);
   println!("{:?}", args.age);
}
