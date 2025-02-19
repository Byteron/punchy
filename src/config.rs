use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "Puncy", about = "A 2.5D side-scroller beatemup.")]
pub struct EngineConfig {
    /// Hot reload assets
    #[structopt(short = "R", long)]
    pub hot_reload: bool,

    /// The directory to load assets from
    #[structopt(short, long)]
    pub asset_dir: Option<String>,

    /// The .game.yaml asset to load at startup
    #[structopt(default_value = "default.game.yaml")]
    pub game_asset: String,
}
