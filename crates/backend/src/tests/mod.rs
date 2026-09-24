//! Backend integration tests.
//!
//! Each submodule exercises one API area end-to-end through the real Axum router
//! (`routes::api_router`) backed by a fresh demo state.

#[cfg(test)]
mod actions;
#[cfg(test)]
mod comments;
#[cfg(test)]
mod contents;
#[cfg(test)]
mod issues;
#[cfg(test)]
mod orgs;
#[cfg(test)]
mod packages;
#[cfg(test)]
mod projects;
#[cfg(test)]
mod pulls;
#[cfg(test)]
mod releases;
#[cfg(test)]
mod repos;
#[cfg(test)]
mod users;
#[cfg(test)]
mod webhooks;
#[cfg(test)]
mod wiki;
