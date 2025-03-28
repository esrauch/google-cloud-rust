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

/// Implements a [DataCatalog](super::stub::DataCatalog) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct DataCatalog<T>
where
    T: super::stub::DataCatalog + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> DataCatalog<T>
where
    T: super::stub::DataCatalog + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::DataCatalog for DataCatalog<T>
where
    T: super::stub::DataCatalog + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn search_catalog(
        &self,
        req: crate::model::SearchCatalogRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchCatalogResponse> {
        self.inner.search_catalog(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_entry_group(
        &self,
        req: crate::model::CreateEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::EntryGroup> {
        self.inner.create_entry_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_entry_group(
        &self,
        req: crate::model::GetEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::EntryGroup> {
        self.inner.get_entry_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_entry_group(
        &self,
        req: crate::model::UpdateEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::EntryGroup> {
        self.inner.update_entry_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_entry_group(
        &self,
        req: crate::model::DeleteEntryGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_entry_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_entry_groups(
        &self,
        req: crate::model::ListEntryGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListEntryGroupsResponse> {
        self.inner.list_entry_groups(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_entry(
        &self,
        req: crate::model::CreateEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Entry> {
        self.inner.create_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_entry(
        &self,
        req: crate::model::UpdateEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Entry> {
        self.inner.update_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_entry(
        &self,
        req: crate::model::DeleteEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_entry(
        &self,
        req: crate::model::GetEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Entry> {
        self.inner.get_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn lookup_entry(
        &self,
        req: crate::model::LookupEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Entry> {
        self.inner.lookup_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_entries(
        &self,
        req: crate::model::ListEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListEntriesResponse> {
        self.inner.list_entries(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn modify_entry_overview(
        &self,
        req: crate::model::ModifyEntryOverviewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::EntryOverview> {
        self.inner.modify_entry_overview(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn modify_entry_contacts(
        &self,
        req: crate::model::ModifyEntryContactsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Contacts> {
        self.inner.modify_entry_contacts(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_tag_template(
        &self,
        req: crate::model::CreateTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagTemplate> {
        self.inner.create_tag_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_tag_template(
        &self,
        req: crate::model::GetTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagTemplate> {
        self.inner.get_tag_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_tag_template(
        &self,
        req: crate::model::UpdateTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagTemplate> {
        self.inner.update_tag_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_tag_template(
        &self,
        req: crate::model::DeleteTagTemplateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_tag_template(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_tag_template_field(
        &self,
        req: crate::model::CreateTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagTemplateField> {
        self.inner.create_tag_template_field(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_tag_template_field(
        &self,
        req: crate::model::UpdateTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagTemplateField> {
        self.inner.update_tag_template_field(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn rename_tag_template_field(
        &self,
        req: crate::model::RenameTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagTemplateField> {
        self.inner.rename_tag_template_field(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn rename_tag_template_field_enum_value(
        &self,
        req: crate::model::RenameTagTemplateFieldEnumValueRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TagTemplateField> {
        self.inner
            .rename_tag_template_field_enum_value(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn delete_tag_template_field(
        &self,
        req: crate::model::DeleteTagTemplateFieldRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_tag_template_field(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_tag(
        &self,
        req: crate::model::CreateTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Tag> {
        self.inner.create_tag(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_tag(
        &self,
        req: crate::model::UpdateTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Tag> {
        self.inner.update_tag(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_tag(
        &self,
        req: crate::model::DeleteTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_tag(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_tags(
        &self,
        req: crate::model::ListTagsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTagsResponse> {
        self.inner.list_tags(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn reconcile_tags(
        &self,
        req: crate::model::ReconcileTagsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.reconcile_tags(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn star_entry(
        &self,
        req: crate::model::StarEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StarEntryResponse> {
        self.inner.star_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn unstar_entry(
        &self,
        req: crate::model::UnstarEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::UnstarEntryResponse> {
        self.inner.unstar_entry(req, options).await
    }

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
    async fn import_entries(
        &self,
        req: crate::model::ImportEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.import_entries(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_config(
        &self,
        req: crate::model::SetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::MigrationConfig> {
        self.inner.set_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn retrieve_config(
        &self,
        req: crate::model::RetrieveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::OrganizationConfig> {
        self.inner.retrieve_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn retrieve_effective_config(
        &self,
        req: crate::model::RetrieveEffectiveConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::MigrationConfig> {
        self.inner.retrieve_effective_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}

/// Implements a [PolicyTagManager](super::stub::PolicyTagManager) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct PolicyTagManager<T>
where
    T: super::stub::PolicyTagManager + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> PolicyTagManager<T>
where
    T: super::stub::PolicyTagManager + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::PolicyTagManager for PolicyTagManager<T>
where
    T: super::stub::PolicyTagManager + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_taxonomy(
        &self,
        req: crate::model::CreateTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Taxonomy> {
        self.inner.create_taxonomy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_taxonomy(
        &self,
        req: crate::model::DeleteTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_taxonomy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_taxonomy(
        &self,
        req: crate::model::UpdateTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Taxonomy> {
        self.inner.update_taxonomy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_taxonomies(
        &self,
        req: crate::model::ListTaxonomiesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTaxonomiesResponse> {
        self.inner.list_taxonomies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_taxonomy(
        &self,
        req: crate::model::GetTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Taxonomy> {
        self.inner.get_taxonomy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_policy_tag(
        &self,
        req: crate::model::CreatePolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PolicyTag> {
        self.inner.create_policy_tag(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_policy_tag(
        &self,
        req: crate::model::DeletePolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_policy_tag(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_policy_tag(
        &self,
        req: crate::model::UpdatePolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PolicyTag> {
        self.inner.update_policy_tag(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_policy_tags(
        &self,
        req: crate::model::ListPolicyTagsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListPolicyTagsResponse> {
        self.inner.list_policy_tags(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_policy_tag(
        &self,
        req: crate::model::GetPolicyTagRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PolicyTag> {
        self.inner.get_policy_tag(req, options).await
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
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
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
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.cancel_operation(req, options).await
    }
}

/// Implements a [PolicyTagManagerSerialization](super::stub::PolicyTagManagerSerialization) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct PolicyTagManagerSerialization<T>
where
    T: super::stub::PolicyTagManagerSerialization + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> PolicyTagManagerSerialization<T>
where
    T: super::stub::PolicyTagManagerSerialization + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::PolicyTagManagerSerialization for PolicyTagManagerSerialization<T>
where
    T: super::stub::PolicyTagManagerSerialization + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn replace_taxonomy(
        &self,
        req: crate::model::ReplaceTaxonomyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Taxonomy> {
        self.inner.replace_taxonomy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn import_taxonomies(
        &self,
        req: crate::model::ImportTaxonomiesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ImportTaxonomiesResponse> {
        self.inner.import_taxonomies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn export_taxonomies(
        &self,
        req: crate::model::ExportTaxonomiesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ExportTaxonomiesResponse> {
        self.inner.export_taxonomies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.cancel_operation(req, options).await
    }
}
