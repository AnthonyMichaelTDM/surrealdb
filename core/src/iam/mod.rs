#[cfg(feature = "policy")]
use cedar_policy::Context;
pub use entities::Level;
use thiserror::Error;

pub mod auth;
pub mod base;
pub mod check;
pub mod clear;
pub mod entities;
pub mod issue;
#[cfg(feature = "jwks")]
pub mod jwks;
#[cfg(feature = "policy")]
pub mod policies;
pub mod signin;
pub mod signup;
pub mod token;
pub mod verify;

pub use self::auth::*;
pub use self::entities::*;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
	#[error("Invalid role '{0}'")]
	InvalidRole(String),

	#[error("Not enough permissions to perform this action")]
	NotAllowed {
		actor: String,
		action: String,
		resource: String,
	},
}

impl From<Error> for String {
	fn from(e: Error) -> String {
		e.to_string()
	}
}

#[cfg(not(feature = "policy"))]
/// Dummy function that always returns Ok(()).
///
/// The only technical change in API is that the `ctx` parameter is now an `Option<()>` instead of `Option<Context>`,
/// as the `Context` type is not available when the `policy` feature is disabled.
/// Surely this won't have far reaching consequences ...
pub fn is_allowed(_: &Actor, _: &Action, _: &Resource, _: Option<()>) -> Result<(), Error> {
	Ok(())
}

#[cfg(feature = "policy")]
pub fn is_allowed(
	actor: &Actor,
	action: &Action,
	resource: &Resource,
	ctx: Option<Context>,
) -> Result<(), Error> {
	match policies::is_allowed(actor, action, resource, ctx.unwrap_or(Context::empty())) {
		(allowed, _) if allowed => Ok(()),
		_ => {
			let err = Error::NotAllowed {
				actor: actor.to_string(),
				action: action.to_string(),
				resource: format!("{}", resource),
			};

			trace!("{}", err);
			Err(err)
		}
	}
}
