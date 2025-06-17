mod resolution_manager;
mod resolutions;

fn main() {
    let (w,h) = resolutions::RESOLUTION_1680_1050;
    let resm = resolution_manager::ResolutionManager::new(w,h,"VGA-1");
    match resm.apply_resolution(){
        Err(error) => println!("{error}"),
        Ok(success) => println!("{success}"),
    }; 
}
