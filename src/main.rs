mod resolution_manager;
mod resolutions;

fn main() {
    use resolutions::RESOLUTION_1680_1050;
    let mut resm = resolution_manager::ResolutionManager::new();
    resm.set_resolution(RESOLUTION_1680_1050.0, RESOLUTION_1680_1050.1);
    resm.apply_resolution();
}
