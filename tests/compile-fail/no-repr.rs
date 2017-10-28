#[macro_use]
extern crate ref_cast;

#[derive(RefCast)] //~ ERROR: proc-macro derive panicked
struct Test {
    s: String, //~^^ HELP: RefCast trait requires #[repr(C)] or #[repr(transparent)]
}

fn main() {}
