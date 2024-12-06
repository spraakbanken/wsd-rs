use std::io;

use ::process_corpus::LemmaToken;
use process_corpus::process_corpus;
use saldo::SaldoLexicon;

pub trait WSDApplication {
    fn disambiguate_text(
        &self,
        text: Vec<Vec<LemmaToken>>,
    ) -> Vec<(Vec<LemmaToken>, Vec<Option<Vec<f32>>>)> {
        let mut out = Vec::new();
        for sen in text {
            out.push(self.disambiguate_sentence(sen));
        }
        out
    }
    fn disambiguate_sentence(
        &self,
        lts: Vec<LemmaToken>,
    ) -> (Vec<LemmaToken>, Vec<Option<Vec<f32>>>) {
        let mut disamb = Vec::new();
        for i in 0..lts.len() {
            disamb.push(self.disambiguate(&lts, i));
        }
        (lts, disamb)
    }
    fn disambiguate(&self, lts: &[LemmaToken], i: usize) -> Option<Vec<f32>>;
}

pub type SharedWSDApplication = Box<dyn WSDApplication>;

pub fn evaluate(_wsd: SharedWSDApplication, _eval_lemmas_file: &str, _eval_key_file: &str) {
    todo!("evaluate is not yet supported")
}

pub fn read_sentences(
    reader: &mut dyn io::BufRead,
    _saldo: Option<&SaldoLexicon>,
    sbxml: bool,
    batch_size: usize,
    _split_mwes: bool,
    _split_compounds: bool,
) -> io::Result<Vec<Vec<LemmaToken>>> {
    let mut out = Vec::new();
    while out.len() < batch_size {
        if sbxml {
            todo!("sbxml format is not yet supported")
        } else {
            let lts = match process_corpus::read_lemma_tokens(reader)? {
                Some(lts) => lts,
                None => return Ok(out),
            };
            out.push(lts);
        }
    }
    Ok(out)
}
