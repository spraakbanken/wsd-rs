use std::{fs, io};

use rstest::{fixture, rstest};

use wsd_application::{
    make_wsd_application,
    wsd_application::{disambiguate_sentences, DisambiguateOptions},
    SharedWSDApplication, TabFormat,
};

#[fixture]
fn sense_model() -> &'static str {
    "assets/sparv-wsd/models/scouse/ALL_512_128_w10_A2_140403_ctx1.bin"
}

#[fixture]
fn context_model() -> &'static str {
    "assets/sparv-wsd/models/scouse/lem_cbow0_s512_w10_NEW2_ctx.bin"
}
#[fixture]
fn vector_wsd(sense_model: &str, context_model: &str) -> SharedWSDApplication {
    let argv = &[
        // "-format=tab".to_string(),
        format!("-svFile={}", sense_model),
        format!("-cvFile={}", context_model),
        "-s1Prior=1".into(),
        "-decay=true".into(),
        "-contextWidth=10".into(),
        "-verbose=false".into(),
    ];
    make_wsd_application(None, "se.gu.spraakbanken.wsd.VectorWSD", argv).expect("VectorWSD created")
}

#[rstest]
fn test_vector_wsd(vector_wsd: SharedWSDApplication) -> eyre::Result<()> {
    let mut reader = io::BufReader::new(fs::File::open("assets/testing/example1.in.txt")?);
    let mut out = Vec::new();
    disambiguate_sentences(
        vector_wsd,
        &mut reader,
        &mut out,
        &TabFormat::default(),
        DisambiguateOptions::default(),
    )?;

    let actual = String::from_utf8(out)?;
    let actual: Vec<&str> = actual.split('\n').collect();
    insta::assert_debug_snapshot!(actual);
    Ok(())
}
