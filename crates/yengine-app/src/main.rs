use yengine_core::Engine;

fn main() {
    if let Err(error) = Engine::new().run()
    {
        eprintln!("Failed to run engine: {}", error);
        std::process::exit(1);
    }
}
