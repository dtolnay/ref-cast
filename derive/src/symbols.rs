use {
    std::fmt::{self, Display},
    syn::{Ident, Path},
};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const REF_CAST: Symbol = Symbol("ref_cast");
pub const CRATE: Symbol = Symbol("crate");

impl Symbol {
    pub fn value(&self) -> &'static str {
        self.0
    }
}

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
