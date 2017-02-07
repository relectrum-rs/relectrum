// Rust Stratum Library
// Written in 2015 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// Modified in 2017 by
//   Jean Pierre Dudey <jeandudey@hotmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Rust Stratum Library
//!
//! Rust support for the Stratum protocol.
//!

// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod client;
pub mod error;

pub use serde_json::value::Value;
pub use error::Error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// A Stratum request object
pub struct Request {
    /// The name of the RPC call
    pub method: String,
    /// Parameters to the RPC call
    pub params: Vec<Value>,
    /// Identifier for this Request, which should appear in the response
    pub id: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// A Stratum response object
pub struct Response {
    /// A result if there is one, or null
    pub result: Option<Value>,
    /// An error if there is one, or null
    pub error: Option<error::RpcError>,
    /// Identifier for this Request, which should match that of the request
    pub id: Value,
}

impl Response {
    /// Extract the result from a response, consuming the response
    pub fn into_result<T: serde::Deserialize>(self) -> Result<T, Error> {
        if let Some(e) = self.error {
            return Err(Error::Rpc(e));
        }
        match self.result {
            Some(res) => serde_json::value::from_value(res).map_err(Error::Json),
            None => Err(Error::NoErrorOrResult),
        }
    }

    /// Return the RPC error, if there was one, but do not check the result
    pub fn check_error(self) -> Result<(), Error> {
        if let Some(e) = self.error {
            Err(Error::Rpc(e))
        } else {
            Ok(())
        }
    }

    /// Returns whether or not the `result` field is empty
    pub fn is_none(&self) -> bool {
        self.result.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::Response;
    use super::serde_json::Value;

    #[test]
    fn response_is_none() {
        let joanna = Response {
            result: Some(Value::Bool(true)),
            error: None,
            id: From::from(81),
        };

        let bill = Response {
            result: None,
            error: None,
            id: From::from(66),
        };

        assert!(!joanna.is_none());
        assert!(bill.is_none());
    }
}
