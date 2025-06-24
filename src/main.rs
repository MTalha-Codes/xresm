// This file is part of xresm.
// Copyright (c) 2025 Muhammad Talha
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
