use std::collections::HashMap;

use crate::{address::CellAddress, cell::Cell};

#[derive(Clone, Debug, Default)]
pub struct Sheet {
    cells: HashMap<CellAddress, Cell>,
}

impl Sheet {
    pub fn new() -> Self {
        Default::default()
    }
}
