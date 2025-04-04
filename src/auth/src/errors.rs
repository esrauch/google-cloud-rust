// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Common errors generated by the components in this crate.

use http::StatusCode;
use std::error::Error;

pub use gax::error::CredentialsError;

/// A helper to create a retryable error.
pub(crate) fn retryable<T: Error + Send + Sync + 'static>(source: T) -> CredentialsError {
    CredentialsError::new(true, source)
}

#[allow(dead_code)]
pub(crate) fn retryable_from_str<T: Into<String>>(message: T) -> CredentialsError {
    CredentialsError::from_str(true, message)
}

/// A helper to create a non-retryable error.
pub(crate) fn non_retryable<T: Error + Send + Sync + 'static>(source: T) -> CredentialsError {
    CredentialsError::new(false, source)
}

pub(crate) fn non_retryable_from_str<T: Into<String>>(message: T) -> CredentialsError {
    CredentialsError::from_str(false, message)
}

pub(crate) fn is_retryable(c: StatusCode) -> bool {
    match c {
        // Internal server errors do not indicate that there is anything wrong
        // with our request, so we retry them.
        StatusCode::INTERNAL_SERVER_ERROR
        | StatusCode::SERVICE_UNAVAILABLE
        | StatusCode::REQUEST_TIMEOUT
        | StatusCode::TOO_MANY_REQUESTS => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(StatusCode::INTERNAL_SERVER_ERROR)]
    #[test_case(StatusCode::SERVICE_UNAVAILABLE)]
    #[test_case(StatusCode::REQUEST_TIMEOUT)]
    #[test_case(StatusCode::TOO_MANY_REQUESTS)]
    fn retryable(c: StatusCode) {
        assert!(is_retryable(c));
    }

    #[test_case(StatusCode::NOT_FOUND)]
    #[test_case(StatusCode::UNAUTHORIZED)]
    #[test_case(StatusCode::BAD_REQUEST)]
    #[test_case(StatusCode::BAD_GATEWAY)]
    #[test_case(StatusCode::PRECONDITION_FAILED)]
    fn non_retryable(c: StatusCode) {
        assert!(!is_retryable(c));
    }

    #[test]
    fn helpers() {
        let e = super::retryable_from_str("test-only-err-123");
        assert!(e.is_retryable(), "{e}");
        assert!(e.source().unwrap().source().is_none());
        let got = format!("{e}");
        assert!(got.contains("test-only-err-123"), "{got}");

        let input = "NaN".parse::<u32>().unwrap_err();
        let e = super::retryable(input.clone());
        assert!(e.is_retryable(), "{e}");
        let got = format!("{e}");
        assert!(got.contains(&format!("{input}")), "{got}");

        let e = super::non_retryable_from_str("test-only-err-123");
        assert!(!e.is_retryable(), "{e}");
        let got = format!("{e}");
        assert!(got.contains("test-only-err-123"), "{got}");

        let e = super::non_retryable(input.clone());
        assert!(!e.is_retryable(), "{e}");
        let got = format!("{e}");
        assert!(got.contains(&format!("{input}")), "{got}");
    }
}
