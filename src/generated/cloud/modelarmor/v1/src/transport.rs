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

/// Implements [ModelArmor](crate::stubs::ModelArmor) using a [gclient::ReqwestClient].
#[derive(Clone)]
pub struct ModelArmor {
    inner: gclient::ReqwestClient,
}

impl std::fmt::Debug for ModelArmor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("ModelArmor")
            .field("inner", &self.inner)
            .finish()
    }
}

impl ModelArmor {
    pub async fn new(config: gclient::ClientConfig) -> Result<Self> {
        let inner = gclient::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::stubs::ModelArmor for ModelArmor {
    async fn list_templates(
        &self,
        req: crate::model::ListTemplatesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTemplatesResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/templates", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("orderBy", &req.order_by)]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn get_template(
        &self,
        req: crate::model::GetTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Template> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn create_template(
        &self,
        req: crate::model::CreateTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Template> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/templates", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("templateId", &req.template_id)]);
        let builder = builder.query(&[("requestId", &req.request_id)]);
        self.inner
            .execute(builder, Some(req.template), options)
            .await
    }

    async fn update_template(
        &self,
        req: crate::model::UpdateTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Template> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.template
                        .as_ref()
                        .ok_or_else(|| gax::path_parameter::missing("template"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .update_mask
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gax::query_parameter::QueryParameter;
                v.add(builder, "updateMask")
            });
        let builder = builder.query(&[("requestId", &req.request_id)]);
        self.inner
            .execute(builder, Some(req.template), options)
            .await
    }

    async fn delete_template(
        &self,
        req: crate::model::DeleteTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("requestId", &req.request_id)]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn get_floor_setting(
        &self,
        req: crate::model::GetFloorSettingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FloorSetting> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn update_floor_setting(
        &self,
        req: crate::model::UpdateFloorSettingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FloorSetting> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.floor_setting
                        .as_ref()
                        .ok_or_else(|| gax::path_parameter::missing("floor_setting"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .update_mask
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gax::query_parameter::QueryParameter;
                v.add(builder, "updateMask")
            });
        self.inner
            .execute(builder, Some(req.floor_setting), options)
            .await
    }

    async fn sanitize_user_prompt(
        &self,
        req: crate::model::SanitizeUserPromptRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SanitizeUserPromptResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:sanitizeUserPrompt", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn sanitize_model_response(
        &self,
        req: crate::model::SanitizeModelResponseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SanitizeModelResponseResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:sanitizeModelResponse", req.name),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/locations", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gclient::NoBody>, options)
            .await
    }
}
