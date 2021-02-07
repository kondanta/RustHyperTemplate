use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Project Saphire")]
pub struct Opt {
    /// Http handler's serving address.
    #[structopt(short = "s", long, default_value = "0.0.0.0:8090")]
    pub http_listen: String,

    /// Verbosity of the logs
    /// Supported values are: ERROR, WARN, INFO, DEBUG and TRACE
    #[structopt(short = "v", long = "verbose")]
    pub log_level: Option<String>,
}

impl Opt {
    pub fn args() -> Self
    where
        Self: Sized,
    {
        Opt::from_args()
    }
}
