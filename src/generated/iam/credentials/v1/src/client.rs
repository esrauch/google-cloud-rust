// Copyright 2025 Google LLC
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
//
// Code generated by sidekick. DO NOT EDIT.
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// An implementation of [crate::traits::IAMCredentials] to make requests with.
///
/// `IAMCredentials` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `IAMCredentials` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `IAMCredentials` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// A service account is a special type of Google account that belongs to your
/// application or a virtual machine (VM), instead of to an individual end user.
/// Your application assumes the identity of the service account to call Google
/// APIs, so that the users aren't directly involved.
///
/// Service account credentials are used to temporarily assume the identity
/// of the service account. Supported credential types include OAuth 2.0 access
/// tokens, OpenID Connect ID tokens, self-signed JSON Web Tokens (JWTs), and
/// more.
#[derive(Clone, Debug)]
pub struct IAMCredentials {
    inner: Arc<dyn crate::traits::dyntraits::IAMCredentials>,
}

impl IAMCredentials {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: crate::traits::IAMCredentials + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::traits::dyntraits::IAMCredentials>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::traits::IAMCredentials> {
        crate::transport::IAMCredentials::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::traits::IAMCredentials> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::IAMCredentials::new)
    }

    /// Generates an OAuth 2.0 access token for a service account.
    pub fn generate_access_token(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::iam_credentials::GenerateAccessToken {
        crate::builders::iam_credentials::GenerateAccessToken::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Generates an OpenID Connect ID token for a service account.
    pub fn generate_id_token(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::iam_credentials::GenerateIdToken {
        crate::builders::iam_credentials::GenerateIdToken::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Signs a blob using a service account's system-managed private key.
    pub fn sign_blob(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::iam_credentials::SignBlob {
        crate::builders::iam_credentials::SignBlob::new(self.inner.clone()).set_name(name.into())
    }

    /// Signs a JWT using a service account's system-managed private key.
    pub fn sign_jwt(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::iam_credentials::SignJwt {
        crate::builders::iam_credentials::SignJwt::new(self.inner.clone()).set_name(name.into())
    }
}
