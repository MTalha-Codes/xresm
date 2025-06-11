use std::process::Command;
pub struct ResolutionManager {
    desired_resolution_width: String, // entered by user
    desired_resolution_height: String, // entered by user
    mode_line: String, // generated  by cvt
    output_source:String, // VGA, HDMI or DP etc
}


impl ResolutionManager {
    pub fn new() -> Self {
        ResolutionManager {
            desired_resolution_height: String::new(), 
            desired_resolution_width: String::new(), 
            mode_line: String::new(), 
            output_source: String::new(), 
        }
    }
    pub fn set_resolution(&mut self, width:&str, height:&str) {
        self.desired_resolution_width = width.to_string();
        self.desired_resolution_height = height.to_string();
    }
    
    pub fn set_output_source(&mut self, output:&str) {
        self.output_source = output.to_string();
    }
    fn is_xrandr_installed(&self) -> (bool,String){
        let command = 
            Command::new("xrandr")
                .output()
                .unwrap();
        let cmd_status = command.status.success();
        let success_message = String::from_utf8(command.stdout).unwrap();
        let error_message = String::from_utf8(command.stderr).unwrap();
        if cmd_status { (cmd_status,success_message) } else { (cmd_status,error_message) }
    }
    fn is_cvt_installed(&self) -> (bool,String) {
        let command = 
           Command::new("cvt")
            .output()
            .unwrap();
        let cmd_status = command.status.success();
        let success_message = String::from_utf8(command.stdout).unwrap();
        let error_message = String::from_utf8(command.stderr).unwrap();
        if cmd_status { (cmd_status,success_message) } else { (cmd_status,error_message) } 
    }
    fn get_mode_line(&mut self) {
        let cvt_status = self.is_cvt_installed();
        if cvt_status.0 {
            println!("cvt is not installed on your system !\nError: {}", cvt_status.1);
            return;
        }
        let cvt_modeline = 
            String::
            from_utf8(Command::
            new("cvt")
            .arg(format!("{} {}",self.desired_resolution_width,self.desired_resolution_height))
            .output()
            .unwrap().stdout)
            .unwrap();
        let double_quote_index = cvt_modeline.find("\"").unwrap();
        let actual_mode_line = String::from(&cvt_modeline[double_quote_index..]);
        self.mode_line = actual_mode_line;
    }
    fn get_output_source(&mut self)
    {
        let xrandr_status = self.is_xrandr_installed();
        if !xrandr_status.0 {
            println!("xrandr is not installed on your system !\nError: {}", xrandr_status.1);
            return;
        }
        let source_line = String::from_utf8(Command::new("xrandr | grep -w connected")
            .output()
            .unwrap().stdout
        ).unwrap();
        let word_connected_index = source_line.find("connected").unwrap();
        self.output_source = String::from(&source_line[..word_connected_index]);
    }
    pub fn apply_resolution(&self) {
       todo!("to be implemented")
    }
}

