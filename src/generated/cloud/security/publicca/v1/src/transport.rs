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

use crate::Result;
#[allow(unused_imports)]
use gax::error::Error;

/// Implements [PublicCertificateAuthorityService](super::stub::PublicCertificateAuthorityService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct PublicCertificateAuthorityService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for PublicCertificateAuthorityService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("PublicCertificateAuthorityService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl PublicCertificateAuthorityService {
    pub async fn new(config: gax::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::PublicCertificateAuthorityService for PublicCertificateAuthorityService {
    async fn create_external_account_key(
        &self,
        req: crate::model::CreateExternalAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ExternalAccountKey> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/externalAccountKeys", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.external_account_key), options)
            .await
    }
}
