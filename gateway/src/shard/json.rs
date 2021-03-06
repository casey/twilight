#[cfg(not(feature = "simd-json"))]
pub use serde_json::{from_slice, from_str, to_string, to_vec, Error as JsonError};
#[cfg(feature = "simd-json")]
pub use simd_json::{from_slice, from_str, to_string, to_vec, Error as JsonError};

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::gateway::event::GatewayEvent;

#[derive(Debug)]
pub enum GatewayEventParsingError {
    /// Deserializing the GatewayEvent payload from JSON failed.
    Deserializing {
        /// Reason for the error.
        source: JsonError,
    },
    /// The payload received from Discord was an unrecognized or invalid
    /// structure.
    ///
    /// The payload was either invalid JSON or did not contain the necessary
    /// "op" key in the object.
    PayloadInvalid,
}

impl Display for GatewayEventParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Deserializing { source } => Display::fmt(source, f),
            Self::PayloadInvalid => f.write_str("payload is an invalid json structure"),
        }
    }
}

impl Error for GatewayEventParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Deserializing { source } => Some(source),
            Self::PayloadInvalid => None,
        }
    }
}

/// Parse a gateway event from a string using `serde_json` with headers.
///
/// # Errors
///
/// Returns [`GatewayEventParsingError::PayloadInvalid`] if the payload wasn't a valid
/// `GatewayEvent` data structure.
///
/// Returns [`GatewayEventParsingError::Deserializing`] if the payload failed to
/// deserialize.
///
/// [`GatewayEventParsingError::PayloadInvalid`]: enum.GatewayEventParsingError.html#variant.PayloadInvalid
/// [`GatewayEventParsingError::Deserializing`]: enum.GatewayEventParsingError.html#variant.Deserializing
#[cfg(not(feature = "simd-json"))]
#[allow(dead_code)]
pub fn parse_gateway_event(
    op: u8,
    sequence: Option<u64>,
    event_type: Option<&str>,
    json: &mut str,
) -> Result<GatewayEvent, GatewayEventParsingError> {
    use serde::de::DeserializeSeed;
    use serde_json::Deserializer;
    use twilight_model::gateway::event::GatewayEventDeserializer;

    let gateway_deserializer = GatewayEventDeserializer::new(op, sequence, event_type);
    let mut json_deserializer = Deserializer::from_str(json);

    gateway_deserializer
        .deserialize(&mut json_deserializer)
        .map_err(|source| {
            tracing::debug!("invalid JSON: {}", json);

            GatewayEventParsingError::Deserializing { source }
        })
}

/// Parse a gateway event from a string using `simd-json` with headers.
///
/// # Errors
///
/// Returns [`GatewayEventParsingError::PayloadInvalid`] if the payload wasn't a valid
/// `GatewayEvent` data structure.
///
/// Returns [`GatewayEventParsingError::Deserializing`] if the payload failed to
/// deserialize.
///
/// [`GatewayEventParsingError::PayloadInvalid`]: enum.GatewayEventParsingError.html#variant.PayloadInvalid
/// [`GatewayEventParsingError::Deserializing`]: enum.GatewayEventParsingError.html#variant.Deserializing
#[allow(unsafe_code)]
#[cfg(feature = "simd-json")]
#[allow(dead_code)]
pub fn parse_gateway_event(
    op: u8,
    sequence: Option<u64>,
    event_type: Option<&str>,
    json: &mut str,
) -> Result<GatewayEvent, GatewayEventParsingError> {
    use serde::de::DeserializeSeed;
    use simd_json::Deserializer;
    use twilight_model::gateway::event::gateway::GatewayEventDeserializer;

    let gateway_deserializer = GatewayEventDeserializer::new(op, sequence, event_type);

    // # Safety
    //
    // The SIMD deserializer may change the string in ways that aren't
    // UTF-8 valid, but that's fine because it won't be used again.
    let json_bytes = unsafe { json.as_bytes_mut() };

    let mut json_deserializer = Deserializer::from_slice(json_bytes)
        .map_err(|_| GatewayEventParsingError::PayloadInvalid)?;

    gateway_deserializer
        .deserialize(&mut json_deserializer)
        .map_err(|source| {
            tracing::debug!("invalid JSON: {}", json);

            GatewayEventParsingError::Deserializing { source }
        })
}

#[cfg(test)]
mod tests {
    use super::GatewayEventParsingError;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};

    assert_fields!(GatewayEventParsingError::Deserializing: source);
    assert_impl_all!(GatewayEventParsingError: Debug, Error, Send, Sync);
}
