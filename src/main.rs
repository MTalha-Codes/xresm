mod resolutions;
mod resolution_manager;

fn main() {
    use resolutions::RESOLUTION_1024_768;
    let mut resm = resolution_manager::ResolutionManager::new();
    resm.set_resolution(RESOLUTION_1024_768.0, RESOLUTION_1024_768.1);
    resm.apply_resolution();
}
