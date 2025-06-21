mod resolution_manager;
mod resolutions;
mod xresm_errors;

fn main() {
    let (w,h) = resolutions::RESOLUTION_1680_1050;
    let resm = resolution_manager::ResolutionManager::new(w,h,"VGA-1");
    match resm.apply_resolution(){
        Err(e) => println!("{}",e),
        Ok(()) => println!("Resolution {}x{} applied successfully !",w,h),
    }; 
}
