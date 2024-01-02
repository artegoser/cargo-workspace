use crate::term::run_term;

pub fn commit(name: &str) {
    run_term("git", vec!["commit", "-m", name]);
}
