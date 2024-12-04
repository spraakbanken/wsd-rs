use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(author,version,about,long_about=None)]
pub struct Args {
    /// load saldo from this file
    #[clap(long)]
    pub saldo: Option<PathBuf>,
    /// app-name to use
    #[clap(long = "appName")]
    pub app_name: String,
    /// Format of the output
    #[clap(long, value_enum, default_value = "sbxml")]
    pub format: Format,
    /// Should MWEs be split?
    #[clap(long = "splitMWEs", value_enum, default_value = "false")]
    pub split_mwes: Bool,
    /// Should Compunds be split?
    #[clap(long = "splitCompounds", value_enum, default_value = "true")]
    pub split_compounds: Bool,
    /// The size of each batch
    #[clap(long = "batchSize", default_value = "1")]
    pub batch_size: u64,
    /// evalLemmas
    #[clap(long = "evalLemmas")]
    pub eval_lemmas: Option<String>,
    /// evalKey
    #[clap(long = "evalKey")]
    pub eval_key: Option<String>,
    /// forLemma
    #[clap(long = "forLemma")]
    pub for_lemma: Option<String>,
    /// The maximum sense
    #[clap(long = "maxSen", default_value = "2147483647")]
    pub max_sen: usize,
    /// Verbosity
    #[clap(long,short='v',action=clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Format {
    Tab,
    Sbxml,
    Eval,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Bool {
    True,
    False,
}
