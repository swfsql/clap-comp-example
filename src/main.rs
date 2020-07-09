#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}
#[paw::main]
fn main(args: Args) {}
