use crate::xresm_errors::XResolutionManagerErrors;
use crate::xresm_errors::XResolutionManagerErrors::{FaultyModeline, MissingDependencyError, NoDisplayDetected, UnknownDisplayDevice};
use std::io::BufRead;
use std::process::Command;
use winit::event_loop;


/// # Resolution Manager
/// This struct is the backbone of whole project, and it decides how the resolution is handled.
/// ## Fields
/// This struct has two fields for storing:
///    - Width of the Resolution that a user want to apply.
///    - Height of the Resolution that a user want to apply.
///
///   Both fields are of type `String`
/// ## Methods
/// This struct handles resolution management via a number public and private methods.
/// Encapsulation is embraced by keeping some methods private.
pub struct ResolutionManager {
    desired_width: u32,  // entered by user
    desired_height: u32, // entered by user
    target_display: String
}

// Implementation of Resolution Manger struct.
impl ResolutionManager {
    pub fn new(w: u32, h:u32, disp: &str) -> ResolutionManager {
        ResolutionManager {
            desired_height: h,
            desired_width: w,
            target_display: disp.to_string()
        }
    }
    pub fn get_highest_safe_resolution(&self, display: &str) {
        let eloop = event_loop::EventLoop::new().expect("failed to initialize instance of EventLoop");
        let monitors = eloop.available_monitors();
    }
    fn is_xrandr_installed(&self) -> Result<(),XResolutionManagerErrors> {
        let xrandr_status = Command::new("xrandr").arg("--help").output().expect("Failed to execute xrandr --help");
        match xrandr_status.status.success() {
            true => Ok(()),
            false => Err(MissingDependencyError(String::from("xrandr")))
        }
    }

    fn is_cvt_installed(&self) -> Result<(),XResolutionManagerErrors> {
        let xrandr_status = Command::new("cvt").arg("--help").output().expect("Failed to execute cvt --help");
        match xrandr_status.status.success() {
            true => Ok(()),
            false => Err(MissingDependencyError(String::from("cvt")))
        }
    }
    fn get_mode_line(&self) -> String {
        match self.is_cvt_installed() {
            Ok(()) => String::from("success"),
            Err(e) => e.to_string(),
        };
        let cvt_modeline = String::from_utf8(
            Command::new("cvt")
                .arg(self.desired_width.to_string())
                .arg(self.desired_height.to_string())
                .output()
                .expect("couldn't execute cvt")
                .stdout,
        )
        .unwrap();
        let double_quote_index = cvt_modeline.find('"').unwrap_or(0);
        String::from(&cvt_modeline[double_quote_index..cvt_modeline.len() - 1])
    }
    fn get_connected_devices(&self) -> Vec<String> {
        match self.is_xrandr_installed() {
            Ok(()) => vec![String::from("success")],
            Err(e) => vec![e.to_string()],
        };
        // take the output of xrandr into this variable; not print to regular stdout
        let binding = Command::new("xrandr")
            .output()
            .expect("couldn't execute xrandr");
        let xrandr_stdout = binding.stdout.lines();
        let mut connected_devices: Vec<String> = Vec::new();
        for each_line in xrandr_stdout {
            let Ok(unwrapped_line) = each_line else {
                continue;
            };
            let Some(connected_index) = unwrapped_line.find("connected") else {
                continue;
            };
            let device = unwrapped_line[..connected_index].trim();
            if device.contains("dis") {
                continue;
            }
            connected_devices.push(device.to_string());
        }
        connected_devices
    }
    fn new_mode(&self, parsed_modeline: Vec<&str>)  -> Result<(),XResolutionManagerErrors> {
        let newmode_status = Command::new("xrandr")
            .arg("--newmode")
            .args(&parsed_modeline)
            .output()
            .expect("Failed to execute xrandr --newmode <mode name> <clock MHz> ...");
        let command_error = XResolutionManagerErrors::CommandError(String::from_utf8(newmode_status.stderr).expect("Conversion \"utf8 -> String\" failed"),"xrandr --newmode".to_string());
        match newmode_status.status.success() {
            true => Ok(()),
            false => Err(command_error)
        }
    }
    fn add_mode(&self, display_name: &str, mode_name: &str) -> Result<(),XResolutionManagerErrors>  {
        let addmode_status = Command::new("xrandr")
            .args(["--addmode", display_name, mode_name])
            .output()
            .expect("Failed to execute xrandr --addmode <mode name> <target display>");
        let command_error = XResolutionManagerErrors::CommandError(String::from_utf8(addmode_status.stderr).expect("Conversion \"utf8 -> String\" failed"),"xrandr --addmode".to_string());
        match addmode_status.status.success() {
            true => Ok(()),
            false => Err(command_error)
        }
    }
    
    fn apply(&self, display_name: &str, mode_name: &str) -> Result<(),XResolutionManagerErrors>{
        let apply_status =Command::new("xrandr")
            .args(["--output", display_name, "--mode", mode_name])
            .output()
            .expect("Failed to execute xrandr --output <target display> --mode <mode name>");
        let command_error = XResolutionManagerErrors::CommandError(String::from_utf8(apply_status.stderr).expect("Conversion \"utf8 -> String\" failed"),"xrandr --output".to_string());
        match apply_status.status.success() {
            true => Ok(()),
            false => Err(command_error)
        }
    }
    fn get_mode_name<'a>(&self, modeline: &'a str) -> &'a str {
        
        &modeline[modeline.find('"').expect("unexpected modeline found !")
            ..modeline.rfind('"').expect("unexpected modeline found !") + 1]
    }
    pub fn apply_resolution(&self) -> Result<(),XResolutionManagerErrors>{
        let connected_devices = self.get_connected_devices();
        let display_name = &self.target_display;
        if !connected_devices.contains(display_name) {
            return Err(UnknownDisplayDevice(display_name.as_str().to_string()));
        }
        let modeline = self.get_mode_line();
        if modeline.is_empty() {
            return Err(FaultyModeline);
        }
        if connected_devices.is_empty() {
            return Err(NoDisplayDetected);
        }
        let mode_name = self.get_mode_name(&modeline);
        self.new_mode(modeline.lines().collect())?;
        self.add_mode(display_name.as_str(), mode_name)?;
        self.apply(display_name.as_str(), mode_name)
    }
}
