use std::{fs, io};

use finalfusion::{
    embeddings::Embeddings, prelude::ReadWord2Vec, storage::NdArray, vocab::SimpleVocab,
};
use ndarray::s;

use crate::{UsageError, WSDApplication};

pub struct VectorWSD {
    decay: bool,
    s1prior: f32,
    context_width: usize,
    id_to_vectors: Embeddings<SimpleVocab, NdArray>,
    form_to_ctx_vec: Embeddings<SimpleVocab, NdArray>,
}

impl VectorWSD {
    pub fn new(argv: &[String]) -> Result<crate::SharedWSDApplication, crate::UsageError> {
        let mut decay = bool::default();
        let mut s1prior = f32::default();
        let mut context_width = usize::default();
        let mut id_to_vectors = None;
        let mut form_to_ctx_vec = None;

        for a in argv {
            if let Some(val) = a.strip_prefix("-decay=") {
                decay = val.parse().map_err(|_err| UsageError::BadValue {
                    param: "-decay=".into(),
                    value: val.to_string(),
                })?;
            } else if let Some(val) = a.strip_prefix("-s1Prior=") {
                s1prior = val.parse().map_err(|_err| UsageError::BadValue {
                    param: "-s1Prior=".into(),
                    value: val.to_string(),
                })?;
            } else if let Some(val) = a.strip_prefix("-contextWidth=") {
                context_width = val.parse().map_err(|_err| UsageError::BadValue {
                    param: "-contextWidth=".into(),
                    value: val.to_string(),
                })?;
            } else if let Some(val) = a.strip_prefix("-svFile=") {
                id_to_vectors =
                    Some(read_sense_vectors(val).map_err(|err| err.with_param("-svFile="))?);
            } else if let Some(val) = a.strip_prefix("-cvFile=") {
                form_to_ctx_vec =
                    Some(read_ctx_vectors(val).map_err(|err| err.with_param("-cvFile="))?);
            }
        }
        Ok(Box::new(Self {
            // saldo,
            decay,
            s1prior,
            context_width,
            id_to_vectors: id_to_vectors
                .ok_or_else(|| UsageError::missing_required_argument("-svFile="))?,
            form_to_ctx_vec: form_to_ctx_vec
                .ok_or_else(|| UsageError::missing_required_argument("-cvFile="))?,
        }))
    }

    fn add_s1prior(&self, ps: &[String], out: &mut [f32], seen: &[bool]) {
        let mut min = i32::MAX;
        for i in 0..out.len() {
            if !seen[i] {
                continue;
            }
            let s = &ps[i];
            let ix = s.rfind("..").expect("a valid lemma_id");
            let id: i32 = s[ix + 2..].parse().unwrap();
            if id < min {
                min = id;
            }
        }
        for i in 0..out.len() {
            if !seen[i] {
                continue;
            }
            let s = &ps[i];
            let ix = s.rfind("..").expect("a valid lemma_id");
            let id: i32 = s[ix + 2..].parse().unwrap();
            if id == min {
                out[i] += self.s1prior;
            }
        }
    }
}

impl WSDApplication for VectorWSD {
    fn disambiguate(&self, lts: &[process_corpus::LemmaToken], i: usize) -> Option<Vec<f32>> {
        let len = lts.len();
        let li = &lts[i];
        let mut out = vec![0f32; li.possible_senses().len()];
        if out.len() < 2 {
            return None;
        }
        // let mut svs = vec![vec![]; out.len()];
        let (svs, seen) = self.id_to_vectors.embedding_batch(li.possible_senses());
        // for (j, poss_sense) in li.possible_senses().iter().enumerate() {
        //     svs[j] = self.id_to_vectors.embedding(&poss_sense);
        // }
        if !seen.iter().any(|x| *x) {
            return None;
        }
        self.add_s1prior(li.possible_senses(), &mut out, &seen);

        let start = 0.max(i - self.context_width);
        let end = (len - 1).min(i + self.context_width);
        for k in start..=end {
            if k == i {
                continue;
            }
            let Some(l) = lts[k].possible_lemmas().get(0) else {
                continue;
            };
            let Some(cv) = self.form_to_ctx_vec.embedding(l) else {
                continue;
            };

            let weight = if self.decay {
                let mut weight = (self.context_width - k.abs_diff(i) + 1) as f32;
                weight /= 2.0 * self.context_width as f32;
                weight
            } else {
                0.5f32 / self.context_width as f32
            };

            for j in 0..out.len() {
                if !seen[j] {
                    continue;
                }
                let sc = svs.slice(s![j, ..]).dot(&cv);
                out[j] += weight * sc;
            }
        }
        normalize_to_probs(&mut out, &seen);
        Some(out)
    }
}

fn normalize_to_probs(out: &mut [f32], seen: &[bool]) {
    let mut m = f32::NEG_INFINITY;
    for i in 0..out.len() {
        if !seen[i] {
            out[i] = 0f32;
        } else if out[i] > m {
            m = out[i];
        }
    }
    if m == f32::NEG_INFINITY {
        return;
    }
    let mut exp_sum = 0f32;
    for i in 0..out.len() {
        if seen[i] {
            exp_sum += (out[i] - m).exp();
        }
    }
    let log_exp_sum = exp_sum.ln() + m;
    for i in 0..out.len() {
        if seen[i] {
            out[i] = (out[i] - log_exp_sum).exp();
        }
    }
}

fn read_sense_vectors(path: &str) -> Result<Embeddings<SimpleVocab, NdArray>, UsageError> {
    log::info!("Reading sense vectors...");
    read_embeddings_from_path(path)
}

fn read_ctx_vectors(path: &str) -> Result<Embeddings<SimpleVocab, NdArray>, UsageError> {
    log::info!("Reading context vectors...");
    read_embeddings_from_path(path)
}

fn read_embeddings_from_path(path: &str) -> Result<Embeddings<SimpleVocab, NdArray>, UsageError> {
    let mut reader =
        io::BufReader::new(fs::File::open(path).map_err(|source| UsageError::IoError {
            param: String::new(),
            path: path.to_string(),
            source,
        })?);
    let embeddings = Embeddings::read_word2vec_binary(&mut reader).map_err(|source| {
        UsageError::Word2VecError {
            param: String::new(),
            path: path.to_string(),
            source,
        }
    })?;

    Ok(embeddings)
}
