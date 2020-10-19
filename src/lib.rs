#[path = "../module_ok.rs"]
mod module_ok;
pub use module_ok::BAR;





mod module_missed;
pub use module_missed::FOO;
