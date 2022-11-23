#![allow(dead_code)]

use std::collections::HashMap;

enum Term {
    Variable(u8),
    Primitive(u8),
    Structure(String, Vec<Term>),
}

fn unify(u: &Term, v: &Term, bindings: &mut HashMap<&Term, &Term>) -> bool {
    false
}

fn unifies_to(u: &Term, v: &Term) -> Result<Term, ()> {
    Err(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
