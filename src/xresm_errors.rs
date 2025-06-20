use thiserror::Error;

#[derive(Error,Debug)]
pub enum XResolutionManagerErrors {
	#[error("{0} not found in ~/usr/bin/")]
	MissingDependencyError(String),
	#[error("Failed to execute {0} due to several issues.")]
	CannotExecuteProgram(String),
	#[error("Failed to apply the resolution {0}x{1}")]
	FailedToApplyResolution(u32,u32),
	#[error("Requested {0}x{1}, but {2} cannot handle resolutions higher than {3}x{4}.")]
	UnsafeResolutionRequested(u32,u32,String,u32,u32),
	#[error("Error caught: {0}\nLogged by: {1}")]
	CommandError(String,String),
}
