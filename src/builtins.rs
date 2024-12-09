pub mod builtins {
    pub fn mcd(args: Vec<&str>) -> std::io::Result<()> {
        std::env::set_current_dir(args[1])
    }
}