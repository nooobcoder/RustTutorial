use messages_actix::MessagesApp;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let app = MessagesApp::new(1035);
    app.run()
}
