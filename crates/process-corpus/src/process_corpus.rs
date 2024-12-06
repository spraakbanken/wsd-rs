use std::io;

use crate::LemmaToken;

pub fn read_lemma_tokens(reader: &mut dyn io::BufRead) -> io::Result<Option<Vec<LemmaToken>>> {
    let mut out = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        let num_read = reader.read_line(&mut line)?;
        if num_read == 0 {
            if out.is_empty() {
                return Ok(None);
            } else {
                return Ok(Some(out));
            }
        }
        let line_trimmed = line.trim();
        if line_trimmed.is_empty() {
            return Ok(Some(out));
        }
        out.push(LemmaToken::parse_line(line_trimmed));
    }
}
