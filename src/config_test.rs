use crate::config::Config;

#[test]
fn load_success() {
    std::env::set_var("OLLAMA_HOST", "localhost");
    std::env::set_var("OLLAMA_PORT", "1234");
    let cfg = Config::load().expect("should load");
    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 1234);
}

#[test]
fn load_missing() {
    std::env::remove_var("OLLAMA_HOST");
    std::env::remove_var("OLLAMA_PORT");
    let res = Config::load();
    assert!(res.is_err());
}
