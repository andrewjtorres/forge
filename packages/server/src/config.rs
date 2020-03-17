use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    /// Database URL to connect to
    #[structopt(env, long)]
    pub database_url: String,

    /// Port number to listen for requests on
    #[structopt(default_value = "8080", env, long, short)]
    pub port: u16,

    /// Host to use
    #[structopt(default_value = "localhost", env, long, short)]
    pub host: String,
}

impl Config {
    pub fn parse() -> Config {
        Config::from_args()
    }
}
