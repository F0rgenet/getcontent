use clap::Parser;


struct Args {
    #[arg(short, long)]
    name: String
}



fn main() {
    println!("{}", 5+2);
}
