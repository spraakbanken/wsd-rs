use std::{borrow::Borrow, fmt};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SaldoId(String);

impl fmt::Display for SaldoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Borrow<String> for SaldoId {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl SaldoId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SaldoEntry {
    id: SaldoId,
    mf: Option<SaldoId>,
    pf: Vec<SaldoId>,
    inv_pf: Vec<SaldoId>,
    inv_mf: Vec<SaldoId>,
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct SaldoEntryBuilder {
    id: Option<SaldoId>,
}

impl SaldoEntry {
    pub fn new(id: SaldoId) -> SaldoEntry {
        Self {
            id,
            mf: None,
            pf: Vec::new(),
            inv_pf: Vec::new(),
            inv_mf: Vec::new(),
        }
    }
    pub fn get_id(&self) -> &SaldoId {
        &self.id
    }
    pub fn set_mf(&mut self, mf: SaldoId) {
        self.mf = Some(mf);
    }
    pub fn add_inv_mf(&mut self, inv_mf: SaldoId) {
        self.inv_mf.push(inv_mf);
    }
    pub fn add_pf(&mut self, pf: SaldoId) {
        self.pf.push(pf);
    }
    pub fn add_inv_pf(&mut self, inv_pf: SaldoId) {
        self.inv_pf.push(inv_pf);
    }
}

impl SaldoEntryBuilder {
    pub fn get_id(&self) -> Option<&SaldoId> {
        self.id.as_ref()
    }
    pub fn set_id(&mut self, id: SaldoId) {
        self.id = Some(id);
    }
    pub fn set_id_opt(&mut self, id_opt: Option<SaldoId>) {
        self.id = id_opt;
    }
    pub fn build(self) -> Result<SaldoEntry, String> {
        let Self { id } = self;
        let Some(id) = id else {
            return Err("missing id".into());
        };
        Ok(SaldoEntry::new(id))
    }
}
