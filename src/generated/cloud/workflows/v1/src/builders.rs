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
use std::sync::Arc;

/// Common implementation for [crate::client::Workflows] request builders.
#[derive(Clone, Debug)]
pub struct WorkflowsRequestBuilder<R: std::default::Default> {
    stub: Arc<dyn crate::traits::dyntraits::Workflows>,
    request: R,
    options: gax::options::RequestOptions,
}

impl<R> WorkflowsRequestBuilder<R>
where
    R: std::default::Default,
{
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Workflows>) -> Self {
        Self {
            stub,
            request: R::default(),
            options: gax::options::RequestOptions::default(),
        }
    }
}

/// The request builder for a Workflows::list_workflows call.
#[derive(Clone, Debug)]
pub struct ListWorkflows(WorkflowsRequestBuilder<crate::model::ListWorkflowsRequest>);

impl ListWorkflows {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Workflows>) -> Self {
        Self(WorkflowsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<crate::model::ListWorkflowsRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<crate::model::ListWorkflowsResponse> {
        self.0
            .stub
            .list_workflows(self.0.request, self.0.options)
            .await
    }

    /// Streams the responses back.
    #[cfg(feature = "unstable-stream")]
    pub async fn stream(
        self,
    ) -> gax::paginator::Paginator<crate::model::ListWorkflowsResponse, gax::error::Error> {
        let token = gax::paginator::extract_token(&self.0.request.page_token);
        let execute = move |token: String| {
            let builder = self.clone();
            builder.0.request.clone().set_page_token(token);
            builder.send()
        };
        gax::paginator::Paginator::new(token, execute)
    }

    /// Sets the value of `parent`.
    pub fn set_parent<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.parent = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
        self.0.request.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.page_token = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.filter = v.into();
        self
    }

    /// Sets the value of `order_by`.
    pub fn set_order_by<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.order_by = v.into();
        self
    }
}

impl gax::options::RequestBuilder for ListWorkflows {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// The request builder for a Workflows::get_workflow call.
#[derive(Clone, Debug)]
pub struct GetWorkflow(WorkflowsRequestBuilder<crate::model::GetWorkflowRequest>);

impl GetWorkflow {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Workflows>) -> Self {
        Self(WorkflowsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<crate::model::GetWorkflowRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<crate::model::Workflow> {
        self.0
            .stub
            .get_workflow(self.0.request, self.0.options)
            .await
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }

    /// Sets the value of `revision_id`.
    pub fn set_revision_id<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.revision_id = v.into();
        self
    }
}

impl gax::options::RequestBuilder for GetWorkflow {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// The request builder for a Workflows::create_workflow call.
#[derive(Clone, Debug)]
pub struct CreateWorkflow(WorkflowsRequestBuilder<crate::model::CreateWorkflowRequest>);

impl CreateWorkflow {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Workflows>) -> Self {
        Self(WorkflowsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<crate::model::CreateWorkflowRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    ///
    /// # Long running operations
    ///
    /// This starts, but does not poll, a longrunning operation. More information
    /// on [create_workflow][crate::client::Workflows::create_workflow].
    pub async fn send(self) -> Result<longrunning::model::Operation> {
        self.0
            .stub
            .create_workflow(self.0.request, self.0.options)
            .await
    }

    /// Sets the value of `parent`.
    pub fn set_parent<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.parent = v.into();
        self
    }

    /// Sets the value of `workflow`.
    pub fn set_workflow<T: Into<Option<crate::model::Workflow>>>(mut self, v: T) -> Self {
        self.0.request.workflow = v.into();
        self
    }

    /// Sets the value of `workflow_id`.
    pub fn set_workflow_id<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.workflow_id = v.into();
        self
    }
}

impl gax::options::RequestBuilder for CreateWorkflow {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// The request builder for a Workflows::delete_workflow call.
#[derive(Clone, Debug)]
pub struct DeleteWorkflow(WorkflowsRequestBuilder<crate::model::DeleteWorkflowRequest>);

impl DeleteWorkflow {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Workflows>) -> Self {
        Self(WorkflowsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<crate::model::DeleteWorkflowRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    ///
    /// # Long running operations
    ///
    /// This starts, but does not poll, a longrunning operation. More information
    /// on [delete_workflow][crate::client::Workflows::delete_workflow].
    pub async fn send(self) -> Result<longrunning::model::Operation> {
        self.0
            .stub
            .delete_workflow(self.0.request, self.0.options)
            .await
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }
}

impl gax::options::RequestBuilder for DeleteWorkflow {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// The request builder for a Workflows::update_workflow call.
#[derive(Clone, Debug)]
pub struct UpdateWorkflow(WorkflowsRequestBuilder<crate::model::UpdateWorkflowRequest>);

impl UpdateWorkflow {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Workflows>) -> Self {
        Self(WorkflowsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<crate::model::UpdateWorkflowRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    ///
    /// # Long running operations
    ///
    /// This starts, but does not poll, a longrunning operation. More information
    /// on [update_workflow][crate::client::Workflows::update_workflow].
    pub async fn send(self) -> Result<longrunning::model::Operation> {
        self.0
            .stub
            .update_workflow(self.0.request, self.0.options)
            .await
    }

    /// Sets the value of `workflow`.
    pub fn set_workflow<T: Into<Option<crate::model::Workflow>>>(mut self, v: T) -> Self {
        self.0.request.workflow = v.into();
        self
    }

    /// Sets the value of `update_mask`.
    pub fn set_update_mask<T: Into<Option<wkt::FieldMask>>>(mut self, v: T) -> Self {
        self.0.request.update_mask = v.into();
        self
    }
}

impl gax::options::RequestBuilder for UpdateWorkflow {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// Common implementation for [crate::client::Locations] request builders.
#[derive(Clone, Debug)]
pub struct LocationsRequestBuilder<R: std::default::Default> {
    stub: Arc<dyn crate::traits::dyntraits::Locations>,
    request: R,
    options: gax::options::RequestOptions,
}

impl<R> LocationsRequestBuilder<R>
where
    R: std::default::Default,
{
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Locations>) -> Self {
        Self {
            stub,
            request: R::default(),
            options: gax::options::RequestOptions::default(),
        }
    }
}

/// The request builder for a Locations::list_locations call.
#[derive(Clone, Debug)]
pub struct ListLocations(LocationsRequestBuilder<location::model::ListLocationsRequest>);

impl ListLocations {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Locations>) -> Self {
        Self(LocationsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<location::model::ListLocationsRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<location::model::ListLocationsResponse> {
        self.0
            .stub
            .list_locations(self.0.request, self.0.options)
            .await
    }

    /// Streams the responses back.
    #[cfg(feature = "unstable-stream")]
    pub async fn stream(
        self,
    ) -> gax::paginator::Paginator<location::model::ListLocationsResponse, gax::error::Error> {
        let token = gax::paginator::extract_token(&self.0.request.page_token);
        let execute = move |token: String| {
            let builder = self.clone();
            builder.0.request.clone().set_page_token(token);
            builder.send()
        };
        gax::paginator::Paginator::new(token, execute)
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.filter = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
        self.0.request.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.page_token = v.into();
        self
    }
}

impl gax::options::RequestBuilder for ListLocations {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// The request builder for a Locations::get_location call.
#[derive(Clone, Debug)]
pub struct GetLocation(LocationsRequestBuilder<location::model::GetLocationRequest>);

impl GetLocation {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Locations>) -> Self {
        Self(LocationsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<location::model::GetLocationRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<location::model::Location> {
        self.0
            .stub
            .get_location(self.0.request, self.0.options)
            .await
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }
}

impl gax::options::RequestBuilder for GetLocation {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// Common implementation for [crate::client::Operations] request builders.
#[derive(Clone, Debug)]
pub struct OperationsRequestBuilder<R: std::default::Default> {
    stub: Arc<dyn crate::traits::dyntraits::Operations>,
    request: R,
    options: gax::options::RequestOptions,
}

impl<R> OperationsRequestBuilder<R>
where
    R: std::default::Default,
{
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Operations>) -> Self {
        Self {
            stub,
            request: R::default(),
            options: gax::options::RequestOptions::default(),
        }
    }
}

/// The request builder for a Operations::list_operations call.
#[derive(Clone, Debug)]
pub struct ListOperations(OperationsRequestBuilder<longrunning::model::ListOperationsRequest>);

impl ListOperations {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Operations>) -> Self {
        Self(OperationsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<longrunning::model::ListOperationsRequest>>(
        mut self,
        v: V,
    ) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<longrunning::model::ListOperationsResponse> {
        self.0
            .stub
            .list_operations(self.0.request, self.0.options)
            .await
    }

    /// Streams the responses back.
    #[cfg(feature = "unstable-stream")]
    pub async fn stream(
        self,
    ) -> gax::paginator::Paginator<longrunning::model::ListOperationsResponse, gax::error::Error>
    {
        let token = gax::paginator::extract_token(&self.0.request.page_token);
        let execute = move |token: String| {
            let builder = self.clone();
            builder.0.request.clone().set_page_token(token);
            builder.send()
        };
        gax::paginator::Paginator::new(token, execute)
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.filter = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
        self.0.request.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.page_token = v.into();
        self
    }
}

impl gax::options::RequestBuilder for ListOperations {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// The request builder for a Operations::get_operation call.
#[derive(Clone, Debug)]
pub struct GetOperation(OperationsRequestBuilder<longrunning::model::GetOperationRequest>);

impl GetOperation {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Operations>) -> Self {
        Self(OperationsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<longrunning::model::GetOperationRequest>>(mut self, v: V) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<longrunning::model::Operation> {
        self.0
            .stub
            .get_operation(self.0.request, self.0.options)
            .await
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }
}

impl gax::options::RequestBuilder for GetOperation {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}

/// The request builder for a Operations::delete_operation call.
#[derive(Clone, Debug)]
pub struct DeleteOperation(OperationsRequestBuilder<longrunning::model::DeleteOperationRequest>);

impl DeleteOperation {
    pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::Operations>) -> Self {
        Self(OperationsRequestBuilder::new(stub))
    }

    /// Set the full request.
    pub fn with_request<V: Into<longrunning::model::DeleteOperationRequest>>(
        mut self,
        v: V,
    ) -> Self {
        self.0.request = v.into();
        self
    }

    /// Sends the request.
    pub async fn send(self) -> Result<wkt::Empty> {
        self.0
            .stub
            .delete_operation(self.0.request, self.0.options)
            .await
    }

    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.0.request.name = v.into();
        self
    }
}

impl gax::options::RequestBuilder for DeleteOperation {
    fn request_options(&mut self) -> &mut gax::options::RequestOptions {
        &mut self.0.options
    }
}
