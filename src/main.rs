mod resolution_manager;
mod resolutions;

fn main() {
    use resolutions::RESOLUTION_1680_1050;
    let (w,h) = (RESOLUTION_1680_1050.0,RESOLUTION_1680_1050.1);
    let resm = resolution_manager::ResolutionManager::new(w,h);
    resm.apply_resolution();
}
