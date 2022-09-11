use std::collections::HashMap;
use std::iter::zip;

enum Typ {
    Cons(TypCons),
    Var(TypVar),
}

type TypVar = u64;

struct TypCons {
    kind: TypVar,
    args: Vec<Typ>,
}

impl PartialEq for TypCons {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && unimplemented!()
    }
}

type TypConstraint = (Typ, Typ);
type TypSubstitutions = HashMap<TypVar, TypCons>;

fn solve(constraints: &[TypConstraint]) -> TypSubstitutions {
    let map = HashMap::new();

    map
}

fn main() {
    println!("Hello, world!");
}

const int_type: TypCons = TypCons {
    kind: 0,
    args: vec![],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves() {
        let constraints = vec![(Typ::Cons(int_type), Typ::Var(0))];
        let result = solve(&constraints);

        assert_eq!(result.get(&0), Some(&int_type));
    }
}
