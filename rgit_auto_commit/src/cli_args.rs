use structopt::StructOpt;

#[derive(StructOpt)]
pub(crate) struct CliArgs {
    #[structopt(name = "dir")]
    base_dir: String,
}