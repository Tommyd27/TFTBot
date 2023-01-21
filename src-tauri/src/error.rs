#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Surreal(#[from] surrealdb::Error),


	#[error("Fail to get Ctx")]
	CtxFail,

	#[error("Value not of type '{0}'")]
	XValueNotOfType(&'static str),

	#[error("Property '{0}' not found")]
	XPropertyNotFound(String),

	#[error("Database Error: '{0}'")]
	DatabaseError(&'static str),
	
}