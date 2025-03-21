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

/// Implements [DatasetService](super::stubs::DatasetService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct DatasetService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for DatasetService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("DatasetService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl DatasetService {
    pub async fn new(config: gax::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stubs::DatasetService for DatasetService {
    async fn get_dataset(
        &self,
        req: crate::model::GetDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dataset> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}",
                    req.project_id, req.dataset_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("datasetView", &req.dataset_view.value())]);
        let builder = builder.query(&[("accessPolicyVersion", &req.access_policy_version)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn insert_dataset(
        &self,
        req: crate::model::InsertDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dataset> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/bigquery/v2/projects/{}/datasets", req.project_id),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("accessPolicyVersion", &req.access_policy_version)]);
        self.inner
            .execute(builder, Some(req.dataset), options)
            .await
    }

    async fn patch_dataset(
        &self,
        req: crate::model::UpdateOrPatchDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dataset> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}",
                    req.project_id, req.dataset_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("accessPolicyVersion", &req.access_policy_version)]);
        self.inner
            .execute(builder, Some(req.dataset), options)
            .await
    }

    async fn update_dataset(
        &self,
        req: crate::model::UpdateOrPatchDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dataset> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PUT,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}",
                    req.project_id, req.dataset_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("accessPolicyVersion", &req.access_policy_version)]);
        self.inner
            .execute(builder, Some(req.dataset), options)
            .await
    }

    async fn delete_dataset(
        &self,
        req: crate::model::DeleteDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::DELETE,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}",
                    req.project_id, req.dataset_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("deleteContents", &req.delete_contents)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn list_datasets(
        &self,
        req: crate::model::ListDatasetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DatasetList> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/bigquery/v2/projects/{}/datasets", req.project_id),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .max_results
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gaxi::query_parameter::QueryParameter;
                v.add(builder, "maxResults")
            });
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("all", &req.all)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn undelete_dataset(
        &self,
        req: crate::model::UndeleteDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dataset> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}:undelete",
                    req.project_id, req.dataset_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }
}

/// Implements [ModelService](super::stubs::ModelService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct ModelService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for ModelService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("ModelService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl ModelService {
    pub async fn new(config: gax::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stubs::ModelService for ModelService {
    async fn get_model(
        &self,
        req: crate::model::GetModelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Model> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/models/{}",
                    req.project_id, req.dataset_id, req.model_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn list_models(
        &self,
        req: crate::model::ListModelsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListModelsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/models",
                    req.project_id, req.dataset_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .max_results
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gaxi::query_parameter::QueryParameter;
                v.add(builder, "maxResults")
            });
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn patch_model(
        &self,
        req: crate::model::PatchModelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Model> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/models/{}",
                    req.project_id, req.dataset_id, req.model_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req.model), options).await
    }

    async fn delete_model(
        &self,
        req: crate::model::DeleteModelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::DELETE,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/models/{}",
                    req.project_id, req.dataset_id, req.model_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }
}

/// Implements [ProjectService](super::stubs::ProjectService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct ProjectService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for ProjectService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("ProjectService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl ProjectService {
    pub async fn new(config: gax::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stubs::ProjectService for ProjectService {
    async fn get_service_account(
        &self,
        req: crate::model::GetServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GetServiceAccountResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/bigquery/v2/projects/{}/serviceAccount", req.project_id),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }
}

/// Implements [RoutineService](super::stubs::RoutineService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct RoutineService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for RoutineService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("RoutineService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl RoutineService {
    pub async fn new(config: gax::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stubs::RoutineService for RoutineService {
    async fn get_routine(
        &self,
        req: crate::model::GetRoutineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Routine> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/routines/{}",
                    req.project_id, req.dataset_id, req.routine_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn insert_routine(
        &self,
        req: crate::model::InsertRoutineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Routine> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/routines",
                    req.project_id, req.dataset_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.routine), options)
            .await
    }

    async fn update_routine(
        &self,
        req: crate::model::UpdateRoutineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Routine> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PUT,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/routines/{}",
                    req.project_id, req.dataset_id, req.routine_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.routine), options)
            .await
    }

    async fn delete_routine(
        &self,
        req: crate::model::DeleteRoutineRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::DELETE,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/routines/{}",
                    req.project_id, req.dataset_id, req.routine_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn list_routines(
        &self,
        req: crate::model::ListRoutinesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRoutinesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/routines",
                    req.project_id, req.dataset_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .max_results
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gaxi::query_parameter::QueryParameter;
                v.add(builder, "maxResults")
            });
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }
}

/// Implements [RowAccessPolicyService](super::stubs::RowAccessPolicyService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct RowAccessPolicyService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for RowAccessPolicyService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("RowAccessPolicyService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl RowAccessPolicyService {
    pub async fn new(config: gax::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stubs::RowAccessPolicyService for RowAccessPolicyService {
    async fn list_row_access_policies(
        &self,
        req: crate::model::ListRowAccessPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRowAccessPoliciesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables/{}/rowAccessPolicies",
                    req.project_id, req.dataset_id, req.table_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn get_row_access_policy(
        &self,
        req: crate::model::GetRowAccessPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RowAccessPolicy> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables/{}/rowAccessPolicies/{}",
                    req.project_id, req.dataset_id, req.table_id, req.policy_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn create_row_access_policy(
        &self,
        req: crate::model::CreateRowAccessPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RowAccessPolicy> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables/{}/rowAccessPolicies",
                    req.project_id, req.dataset_id, req.table_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.row_access_policy), options)
            .await
    }

    async fn update_row_access_policy(
        &self,
        req: crate::model::UpdateRowAccessPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RowAccessPolicy> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PUT,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables/{}/rowAccessPolicies/{}",
                    req.project_id, req.dataset_id, req.table_id, req.policy_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.row_access_policy), options)
            .await
    }

    async fn delete_row_access_policy(
        &self,
        req: crate::model::DeleteRowAccessPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::DELETE,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables/{}/rowAccessPolicies/{}",
                    req.project_id, req.dataset_id, req.table_id, req.policy_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .force
            .iter()
            .fold(builder, |builder, p| builder.query(&[("force", p)]));
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn batch_delete_row_access_policies(
        &self,
        req: crate::model::BatchDeleteRowAccessPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables/{}/rowAccessPolicies:batchDelete",
                    req.project_id, req.dataset_id, req.table_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }
}

/// Implements [TableService](super::stubs::TableService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct TableService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for TableService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("TableService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl TableService {
    pub async fn new(config: gax::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stubs::TableService for TableService {
    async fn get_table(
        &self,
        req: crate::model::GetTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Table> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables/{}",
                    req.project_id, req.dataset_id, req.table_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("selectedFields", &req.selected_fields)]);
        let builder = builder.query(&[("view", &req.view.value())]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn insert_table(
        &self,
        req: crate::model::InsertTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Table> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables",
                    req.project_id, req.dataset_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req.table), options).await
    }

    async fn patch_table(
        &self,
        req: crate::model::UpdateOrPatchTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Table> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables/{}",
                    req.project_id, req.dataset_id, req.table_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("autodetectSchema", &req.autodetect_schema)]);
        self.inner.execute(builder, Some(req.table), options).await
    }

    async fn update_table(
        &self,
        req: crate::model::UpdateOrPatchTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Table> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PUT,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables/{}",
                    req.project_id, req.dataset_id, req.table_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("autodetectSchema", &req.autodetect_schema)]);
        self.inner.execute(builder, Some(req.table), options).await
    }

    async fn delete_table(
        &self,
        req: crate::model::DeleteTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::DELETE,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables/{}",
                    req.project_id, req.dataset_id, req.table_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn list_tables(
        &self,
        req: crate::model::ListTablesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TableList> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/bigquery/v2/projects/{}/datasets/{}/tables",
                    req.project_id, req.dataset_id
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .max_results
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gaxi::query_parameter::QueryParameter;
                v.add(builder, "maxResults")
            });
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }
}
