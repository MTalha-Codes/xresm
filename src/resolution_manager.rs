use std::io::BufRead;
use std::process::{Command, Output};
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
    fn construct_errors_from_cmd_status(&self,cmd_status: Output) -> Result<String,String>
    {
       if cmd_status.status.success() {
           Ok(String::from_utf8(cmd_status.stdout).unwrap())
       } else {
           Err(String::from_utf8(cmd_status.stderr).unwrap())
       }
    }
    fn is_xrandr_installed(&self) -> Result<String,String>{
        let xrandr_status = Command::new("xrandr").arg("--help").output().expect("Failed to execute cvt --help");
        self.construct_errors_from_cmd_status(xrandr_status)
    }

    fn is_cvt_installed(&self) -> Result<String,String> {
        let cvt_status = Command::new("cvt").arg("--help").output().expect("Failed to execute cvt --help");
        self.construct_errors_from_cmd_status(cvt_status)
    }
    fn get_mode_line(&self) -> String {
        self.is_cvt_installed().unwrap_or_else(|e| e);
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
       let xrandr = self.is_xrandr_installed();
        match xrandr { 
            Ok(s) => vec![s],
            Err(e) => vec![e],
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
    fn new_mode(&self, parsed_modeline: Vec<&str>)  -> Result<String,String> {
        let newmode_status = Command::new("xrandr")
            .arg("--newmode")
            .args(&parsed_modeline)
            .output()
            .expect("Failed to execute xrandr --newmode <mode name> <clock MHz> ...");
        self.construct_errors_from_cmd_status(newmode_status)
       
    }
    fn add_mode(&self, display_name: &str, mode_name: &str) -> Result<String,String>  {
        let addmode_status = Command::new("xrandr")
            .args(["--addmode", display_name, mode_name])
            .output()
            .expect("Failed to execute xrandr --addmode <mode name> <target display>");
        self.construct_errors_from_cmd_status(addmode_status)
    }
    
    fn apply(&self, display_name: &str, mode_name: &str) -> Result<String,String>{
        let apply_status =Command::new("xrandr")
            .args(["--output", display_name, "--mode", mode_name])
            .output()
            .expect("Failed to execute xrandr --output <target display> --mode <mode name>"); 
        self.construct_errors_from_cmd_status(apply_status)
    }
    fn get_mode_name<'a>(&self, modeline: &'a str) -> &'a str {
        
        &modeline[modeline.find('"').expect("mode line is bad")
            ..modeline.rfind('"').expect("mode line is bad") + 1]
    }
    pub fn apply_resolution(&self) -> Result<String,String>{
        let connected_devices = self.get_connected_devices();
        let display_name = &self.target_display;
        if !connected_devices.contains(display_name) {
            let error_message = format!("{display_name} not found in the list of connected devices");
            return Err(error_message);
        }
        let modeline = self.get_mode_line();
        if modeline.is_empty() {
            return Err("Modeline is faulty and contain issues".to_string());
        }
        if connected_devices.is_empty() {
            return Err("Couldn't get information about connected devices !".to_string());
        }
        let mode_name = self.get_mode_name(&modeline);
        let new_mode_status  = self.new_mode(modeline.lines().collect())?;
        println!("{new_mode_status}");
        let add_mode_status = self.add_mode(display_name.as_str(), mode_name)?;
        println!("{add_mode_status}");
        self.apply(display_name.as_str(), mode_name)
    }
}
