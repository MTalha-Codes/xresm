use std::process::{Command, Stdio};
pub struct ResolutionManager {
    desired_resolution_width: String, // entered by user
    desired_resolution_height: String, // entered by user
}


impl ResolutionManager {
    pub fn new() -> Self {
        ResolutionManager {
            desired_resolution_height: String::new(), 
            desired_resolution_width: String::new(),
        }
    }
    pub fn set_resolution(&mut self, width:&str, height:&str) {
        self.desired_resolution_width = width.to_string();
        self.desired_resolution_height = height.to_string();
    }
    
    fn is_xrandr_installed(&self) -> (bool,&str){
        (Command::new("xrandr").arg("--help").output().is_ok(),"xrandr not installed on your system")
    }
    fn is_cvt_installed(&self) -> (bool,&str) {
        (Command::new("cvt").arg("--help").output().is_ok(),"cvt not installed on your system")
    }
    fn get_mode_line(&self) -> String {
        {
            let cvt_status = self.is_cvt_installed();
            if !cvt_status.0 {
                println!("cvt is not installed on your system !\nError: {}",cvt_status.1);
                return String::new(); // return empty string;
            }
        }
        let cvt_modeline =String::
        from_utf8(Command::
        new("cvt")
            .arg(&self.desired_resolution_width)
            .arg(&self.desired_resolution_height)
            .output()
            .expect("couldn't run cvt").stdout)
            .unwrap();
        let double_quote_index = cvt_modeline.find('"').unwrap_or(0);
        String::from(&cvt_modeline[double_quote_index..cvt_modeline.len()-1])
    }
    fn get_output_source(&self) -> String {
        {
            let xrandr_status = self.is_xrandr_installed();
            if !xrandr_status.0 {
                println!("xrandr is not installed on your system !\nError: {}",xrandr_status.1);
                return String::new();
            }
        }
        let xrandr_stdout = Command::new("xrandr")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to run xrandr; Reason: Unknown")
            .stdout
            .take()
            .expect("Couldn't access xrandr output");
        let grep = Command::new("grep")
            .args(["-w","connected"])
            .stdin(Stdio::from(xrandr_stdout))
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to run grep; Reason: Unknown")
            .stdout;
        let source_line = String::from_utf8(grep).expect("failed to parse utf-8 byte sequence");
        let connected_index = source_line
            .find("connected")
            .expect("source is parsed incorrectly");
        String::from(&source_line[..connected_index-1])
    }
    
    fn new_mode(&self,modeline: String)
    {
        let _ = Command::new("xrandr")
            .arg("--newmode")
            .arg(modeline)
            .status()
            .expect("newmode returned something bad");
    }
    
    fn add_mode(&self,source: String, mode: String)
    {
        let _ = Command::new("xrandr")
            .arg("--addmode")
            .arg(source)
            .arg(mode)
            .status()
            .expect("addmode returned something bad");
    }
    fn apply_output(&self,source:String, mode:String)
    {
         let _ = Command::new("xrandr")
            .arg("--output")
            .arg(source)
            .arg("--mode")
            .arg(mode)
            .status()
            .expect("output returned something bad");
    }
    pub fn apply_resolution(&self) {
        // This will not be persistent
        let source = self.get_output_source();
        let modeline = self.get_mode_line();
        let mode = String::from(&modeline[modeline.find('"').expect("mode line is bad")+1..modeline.rfind('"').expect("mode line is bad")]);
        self.new_mode(modeline);
        self.add_mode(source.clone(),mode.clone());
        self.apply_output(source,mode);
    }
}

