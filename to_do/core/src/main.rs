use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// first name of user
    first_name: String,
    /// last name of user
    last_name: String,
    /// age of the user
    #[arg(short, long, default_value_t = 1)]
    age: u8,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
