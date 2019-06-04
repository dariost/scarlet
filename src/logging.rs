pub fn init() {
    #[cfg(target_os = "emscripten")]
    console_log::init_with_level(log::Level::Debug).unwrap();
    #[cfg(not(target_os = "emscripten"))]
    pretty_env_logger::init();
}
