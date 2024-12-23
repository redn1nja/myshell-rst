mod builtins;
mod execution;
mod rl;


fn main() {
    let mut rl = rl::create_rl().unwrap();
    execution::main_loop(&mut rl);
}



