/*
Created By: Muhammad Talha (MTalha-Codes)
File Name: constants.rs
Purpose: To store all xresm wide constants
*/


/// **```pub mod supported_resolutions```**
/// 
/// This `mod` contains the supported resolutions defined as constants. They are listed below:
/// 1. 640 x 480
/// 2. 800 x 600
/// 3. 1024 x 768
/// 4. 1280 x 1024
/// 5. 1280 x 800
/// 6. 1366 x 768
/// 7. 1600 x 900
/// 8. 1680 x 1050
/// 9. 1920 x 1080
/// 10. 1920 x 1200
/// 11. 2560 x 1440
/// 12. 2560 x 1600
/// 13. 3840 x 2160
pub mod supported_resolutions {

    pub const RESOLUTION_640_480: (&str,&str) = ("640", "480");
    pub const RESOLUTION_800_600: (&str,&str) = ("800","600");
    pub const RESOLUTION_1024_768: (&str,&str) = ("1024","768");
    pub const RESOLUTION_1280_1024: (&str,&str) = ("1280","1024");
    pub const RESOLUTION_1280_800: (&str,&str) = ("1280","800");
    pub const RESOLUTION_1366_768: (&str,&str) = ("1366","768");
    pub const RESOLUTION_1600_900: (&str,&str) = ("1600","900");
    pub const RESOLUTION_1680_1050: (&str,&str) = ("1680","1050");
    pub const RESOLUTION_1920_1080: (&str,&str) = ("1920","1080");
    pub const RESOLUTION_1920_1200: (&str,&str) = ("1920","1200");
    pub const RESOLUTION_2560_1440: (&str,&str) = ("2560","1440");
    pub const RESOLUTION_2560_1600: (&str,&str) = ("2560","1600");
    pub const RESOLUTION_3840_2160:(&str,&str) = ("3840","2160");
}

pub mod refresh_rates {
    pub const REFRESH_RATE_60:i32 = 60;
    pub const REFRESH_RATE_75:i32 = 75;
    pub const REFRESH_RATE_90:i32 = 90;
    pub const REFRESH_RATE_120:i32 = 120;
    pub const REFRESH_RATE_144:i32 = 144;
    pub const REFRESH_RATE_165:i32 = 165;
    pub const REFRESH_RATE_240:i32 = 240;
    
}