use boa_engine::Script;
use std::clone::Clone;

#[derive(Debug, Clone)]
pub struct Expr {
    pub js_code: String,
}

pub struct ParsedExpr {
    pub script: boa_engine::Script,
}

impl Expr {
    pub fn new(js_code: String) -> Self {
        Self { js_code }
    }
}

impl ParsedExpr {
    pub fn new(script: Script) -> Self {
        Self { script }
    }
}
