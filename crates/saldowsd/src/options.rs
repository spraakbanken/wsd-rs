use wsd_application::UsageError;

#[derive(Debug)]
pub struct Args {
    /// load saldo from this file
    pub saldo: Option<String>,
    /// app-name to use
    pub app_name: String,
    /// Format of the output
    pub sbxml: bool,
    pub eval: bool,
    /// Should MWEs be split?
    pub split_mwes: bool,
    /// Should Compunds be split?
    pub split_compounds: bool,
    /// The size of each batch
    pub batch_size: usize,
    /// evalLemmas
    pub eval_lemmas: Option<String>,
    /// evalKey
    pub eval_key: Option<String>,
    /// forLemma
    pub for_lemma: Option<String>,
    /// The maximum sense
    pub max_sen: usize,
    /// Verbosity
    pub verbose: u8,
}

impl Args {
    pub fn parse(argv: &[String]) -> Result<Self, UsageError> {
        let mut saldo = None;
        let mut app_name_opt = None;
        let mut sbxml = true;
        let mut eval = false;
        let mut split_mwes = false;
        let mut split_compounds = true;
        let mut batch_size = 1;
        let mut eval_lemmas = None;
        let mut eval_key = None;
        let mut max_sen = u32::MAX as usize;
        let mut for_lemma = None;
        let mut verbose = 0;

        for a in argv {
            if let Some(saldo_file) = a.strip_prefix("-saldo=") {
                saldo = Some(saldo_file.to_string());
            } else if let Some(app_name) = a.strip_prefix("-appName=") {
                app_name_opt = Some(app_name.to_string());
            } else if a == "-format=tab" {
                sbxml = false;
            } else if a == "-format=sbxml" {
                sbxml = true;
            } else if a == "-format=eval" {
                eval = true;
            } else if let Some(val) = a.strip_prefix("-splitMWEs=") {
                split_mwes = val.parse().map_err(|_| UsageError::BadValue {
                    param: "-splitMWEs".into(),
                    value: val.into(),
                })?;
            } else if let Some(val) = a.strip_prefix("-splitCompounds=") {
                split_compounds = val.parse().map_err(|_| UsageError::BadValue {
                    param: "-splitCompounds".into(),
                    value: val.into(),
                })?;
            } else if let Some(val) = a.strip_prefix("-batchSize=") {
                batch_size = val.parse().map_err(|_| UsageError::BadValue {
                    param: "-batchSize".into(),
                    value: val.into(),
                })?;
            } else if let Some(val) = a.strip_prefix("-maxSen=") {
                max_sen = val.parse().map_err(|_| UsageError::BadValue {
                    param: "-maxSen".into(),
                    value: val.into(),
                })?;
            } else if let Some(val) = a.strip_prefix("-evalLemmas=") {
                eval_lemmas = Some(val.to_string());
            } else if let Some(val) = a.strip_prefix("-evalKey=") {
                eval_key = Some(val.to_string());
            } else if let Some(val) = a.strip_prefix("-forLemma=") {
                for_lemma = Some(val.to_string());
            } else if a == "-verbose" || a == "-v" {
                verbose += 1;
            } else if a == "-verbose=true" {
                verbose = 2;
            }
        }
        if eval && eval_lemmas.is_none() {
            return Err(UsageError::missing_required_argument(
                "-evalLemmas not specified, required when --format=eval",
            ));
        }
        if eval && eval_key.is_none() {
            return Err(UsageError::missing_required_argument(
                "-evalKey not specified, required when --format=eval",
            ));
        }
        if eval && for_lemma.is_none() {
            return Err(UsageError::missing_required_argument(
                "-forLemma not specified, required when --format=eval",
            ));
        }
        Ok(Self {
            saldo,
            app_name: app_name_opt
                .ok_or_else(|| UsageError::missing_required_argument("-appName not specified"))?,
            sbxml,
            eval,
            split_mwes,
            split_compounds,
            batch_size,
            eval_lemmas,
            eval_key,
            for_lemma,
            max_sen,
            verbose,
        })
    }
}
