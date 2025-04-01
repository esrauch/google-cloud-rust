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

/// Implements a [ContainerAnalysis](super::stub::ContainerAnalysis) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ContainerAnalysis<T>
where
    T: super::stub::ContainerAnalysis + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ContainerAnalysis<T>
where
    T: super::stub::ContainerAnalysis + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::ContainerAnalysis for ContainerAnalysis<T>
where
    T: super::stub::ContainerAnalysis + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_vulnerability_occurrences_summary(
        &self,
        req: crate::model::GetVulnerabilityOccurrencesSummaryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::VulnerabilityOccurrencesSummary> {
        self.inner
            .get_vulnerability_occurrences_summary(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn export_sbom(
        &self,
        req: crate::model::ExportSBOMRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ExportSBOMResponse> {
        self.inner.export_sbom(req, options).await
    }
}
