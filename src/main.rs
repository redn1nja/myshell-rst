mod rl;
mod execution;
mod builtins;

fn main() {
    let mut rl = rl::create_rl().unwrap();
    execution::main_loop(&mut rl);
}



