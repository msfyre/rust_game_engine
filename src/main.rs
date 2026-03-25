use crate::runtime::Runtime;

mod runtime;

fn main() {
    let mut runtime = Runtime::init().unwrap();
    runtime.execute();
}
