use clap::Parser;
#[derive(Parser, Clone)]
pub struct Args {
    #[clap(long, env, default_value = "mysql://root:root@localhost:3306/xmrbank")]
    pub database_url: String,
}
