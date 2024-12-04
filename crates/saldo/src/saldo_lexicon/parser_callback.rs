use std::sync::{Arc, RwLock};

use hashbrown::{HashMap, HashSet};

use crate::{
    saldo_entry::{SaldoEntry, SaldoEntryBuilder, SaldoId},
    shared::xml_reader::{AttributeMap, ContentHandler},
};

#[derive(Debug, Clone, Default)]
pub struct SaldoParserCallback {
    pub curr_entry: Option<SaldoEntryBuilder>,
    pub curr_pos: Option<String>,
    pub curr_lemgram_id: Option<String>,
    pub curr_para: Option<String>,
    pub curr_wf: Option<String>,
    pub curr_relation_target: Option<String>,
    pub all_entries: Vec<SaldoId>,
    pub entries: HashMap<SaldoId, SaldoEntry>,
    pub mfids: HashMap<SaldoId, String>,
    pub pfids: HashMap<SaldoId, HashSet<String>>,
}

impl SaldoParserCallback {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ContentHandler for SaldoParserCallback {
    fn start_element(&mut self, name: &[u8], attributes: AttributeMap) {
        match name {
            b"LexicalEntry" => {
                self.curr_entry = Some(SaldoEntryBuilder::default());
            }
            b"Sense" => {
                if let Some(entry) = &mut self.curr_entry {
                    let id = String::from_utf8(attributes.get(&b"id"[..]).unwrap().to_vec())
                        .expect("valid attr");
                    if entry.get_id().is_some() {
                        todo!("handle sense id already exists")
                    }
                    let id = SaldoId::new(id);
                    entry.set_id(id);
                } else {
                    todo!("handle no curr_entry");
                }
            }
            b"SenseRelation" => {
                self.curr_relation_target = Some(
                    String::from_utf8(attributes.get(&b"targets"[..]).unwrap().to_vec())
                        .expect("valid attr"),
                );
            }
            b"feat" => match attributes.get(&b"att"[..]).map(|s| s.as_ref()) {
                Some(b"label") => {
                    if let Some(entry) = &mut self.curr_entry {
                        let Some(entry_id) = entry.get_id() else {
                            todo!("handle sense id is missing")
                        };
                        let Some(curr_relation_target) = self.curr_relation_target.take() else {
                            todo!("handle")
                        };
                        let rel_type =
                            std::str::from_utf8(attributes.get(&b"val"[..]).unwrap().as_ref())
                                .expect("valid attr");
                        if rel_type == "primary" {
                            if self.mfids.contains_key(entry_id) {
                                todo!("handle entry_id = {} already exists", entry_id);
                            }
                            self.mfids.insert(entry_id.clone(), curr_relation_target);
                        } else if rel_type == "secondary" {
                            self.pfids
                                .entry(entry_id.clone())
                                .or_default()
                                .insert(curr_relation_target);
                        }
                    } else {
                        todo!("handle no curr_entry");
                    }
                }
                Some(b"language") => (),
                Some(b"languageCoding") => (),
                Some(b"lemgram") => {
                    self.curr_lemgram_id =
                        Some(String::from_utf8(attributes[&b"val"[..]].to_vec()).unwrap())
                }
                Some(b"paradigm") => {
                    self.curr_para =
                        Some(String::from_utf8(attributes[&b"val"[..]].to_vec()).unwrap())
                }
                Some(b"partOfSpeech") => {
                    self.curr_pos =
                        Some(String::from_utf8(attributes[&b"val"[..]].to_vec()).unwrap())
                }
                Some(b"writtenForm") => {
                    self.curr_wf =
                        Some(String::from_utf8(attributes[&b"val"[..]].to_vec()).unwrap())
                }
                Some(x) => todo!("handle att='{}'", String::from_utf8_lossy(x)),
                None => todo!("handle att is missing"),
            },
            _ => (),
        }
    }
    fn end_element(&mut self, name: &[u8]) {
        match name {
            b"LexicalEntry" => {
                if let Some(entry_builder) = self.curr_entry.take() {
                    match entry_builder.build() {
                        Ok(entry) => {
                            let entry_id = entry.get_id().clone();
                            self.all_entries.push(entry_id.clone());
                            self.entries.insert(entry_id, entry);
                        }
                        Err(err) => todo!("handle error={:?}", err),
                    }
                }
            }
            _ => (),
        }
    }
}
