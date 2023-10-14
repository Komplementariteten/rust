use structopt::StructOpt;

#[derive(StructOpt)]
pub(crate) struct CliArgs {
    #[structopt(name = "dir", long)]
    pub(crate) base_dir: Option<String>,
    #[structopt(name = "help", long)]
    pub(crate) print_help: bool,
}