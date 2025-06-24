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


use thiserror::Error;

#[derive(Error,Debug)]
pub enum XResolutionManagerErrors {
	#[error("{0} not found in ~/usr/bin/")]
	MissingDependencyError(String),
	#[error("Failed to execute {0} due to several issues.")]
	CannotExecuteProgramError(String),
	#[error("Failed to apply the resolution {0}x{1}")]
	FailedToApplyResolution(u32,u32),
	#[error("Requested {0}x{1}, but {2} cannot handle resolutions higher than {3}x{4}.")]
	UnsafeResolutionRequested(u32,u32,String,u32,u32),
	#[error("Error caught: {0}\nLogged by: {1}")]
	CommandError(String,String),
	#[error("{0} not found in list of connected display devices")]
	UnknownDisplayDevice(String),
	#[error("Modeline is either empty or error-prone !")]
	FaultyModeline,
	#[error("No Display Devices Detected")]
	NoDisplayDetected,
}
