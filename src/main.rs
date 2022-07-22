pub mod log;
pub mod cfg;

fn main() {
    log::init();
    cfg::init()
}
