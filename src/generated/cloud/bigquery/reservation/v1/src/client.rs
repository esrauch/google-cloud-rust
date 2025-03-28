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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the BigQuery Reservation API.
///
/// # Service Description
///
/// This API allows users to manage their BigQuery reservations.
///
/// A reservation provides computational resource guarantees, in the form of
/// [slots](https://cloud.google.com/bigquery/docs/slots), to users. A slot is a
/// unit of computational power in BigQuery, and serves as the basic unit of
/// parallelism. In a scan of a multi-partitioned table, a single slot operates
/// on a single partition of the table. A reservation resource exists as a child
/// resource of the admin project and location, e.g.:
/// `projects/myproject/locations/US/reservations/reservationName`.
///
/// A capacity commitment is a way to purchase compute capacity for BigQuery jobs
/// (in the form of slots) with some committed period of usage. A capacity
/// commitment resource exists as a child resource of the admin project and
/// location, e.g.:
/// `projects/myproject/locations/US/capacityCommitments/id`.
///
/// # Configuration
///
/// `ReservationService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `ReservationService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ReservationService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ReservationService {
    inner: Arc<dyn super::stub::dynamic::ReservationService>,
}

impl ReservationService {
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
        T: super::stub::ReservationService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::ReservationService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ReservationService> {
        super::transport::ReservationService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ReservationService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::ReservationService::new)
    }

    /// Creates a new reservation resource.
    pub fn create_reservation(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::CreateReservation {
        super::builder::reservation_service::CreateReservation::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists all the reservations for the project in the specified location.
    pub fn list_reservations(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::ListReservations {
        super::builder::reservation_service::ListReservations::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns information about the reservation.
    pub fn get_reservation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::GetReservation {
        super::builder::reservation_service::GetReservation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a reservation.
    /// Returns `google.rpc.Code.FAILED_PRECONDITION` when reservation has
    /// assignments.
    pub fn delete_reservation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::DeleteReservation {
        super::builder::reservation_service::DeleteReservation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates an existing reservation resource.
    pub fn update_reservation(
        &self,
        reservation: impl Into<crate::model::Reservation>,
    ) -> super::builder::reservation_service::UpdateReservation {
        super::builder::reservation_service::UpdateReservation::new(self.inner.clone())
            .set_reservation(reservation.into())
    }

    /// Fail over a reservation to the secondary location. The operation should be
    /// done in the current secondary location, which will be promoted to the
    /// new primary location for the reservation.
    /// Attempting to failover a reservation in the current primary location will
    /// fail with the error code `google.rpc.Code.FAILED_PRECONDITION`.
    pub fn failover_reservation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::FailoverReservation {
        super::builder::reservation_service::FailoverReservation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new capacity commitment resource.
    pub fn create_capacity_commitment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::CreateCapacityCommitment {
        super::builder::reservation_service::CreateCapacityCommitment::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists all the capacity commitments for the admin project.
    pub fn list_capacity_commitments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::ListCapacityCommitments {
        super::builder::reservation_service::ListCapacityCommitments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns information about the capacity commitment.
    pub fn get_capacity_commitment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::GetCapacityCommitment {
        super::builder::reservation_service::GetCapacityCommitment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a capacity commitment. Attempting to delete capacity commitment
    /// before its commitment_end_time will fail with the error code
    /// `google.rpc.Code.FAILED_PRECONDITION`.
    pub fn delete_capacity_commitment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::DeleteCapacityCommitment {
        super::builder::reservation_service::DeleteCapacityCommitment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates an existing capacity commitment.
    ///
    /// Only `plan` and `renewal_plan` fields can be updated.
    ///
    /// Plan can only be changed to a plan of a longer commitment period.
    /// Attempting to change to a plan with shorter commitment period will fail
    /// with the error code `google.rpc.Code.FAILED_PRECONDITION`.
    pub fn update_capacity_commitment(
        &self,
        capacity_commitment: impl Into<crate::model::CapacityCommitment>,
    ) -> super::builder::reservation_service::UpdateCapacityCommitment {
        super::builder::reservation_service::UpdateCapacityCommitment::new(self.inner.clone())
            .set_capacity_commitment(capacity_commitment.into())
    }

    /// Splits capacity commitment to two commitments of the same plan and
    /// `commitment_end_time`.
    ///
    /// A common use case is to enable downgrading commitments.
    ///
    /// For example, in order to downgrade from 10000 slots to 8000, you might
    /// split a 10000 capacity commitment into commitments of 2000 and 8000. Then,
    /// you delete the first one after the commitment end time passes.
    pub fn split_capacity_commitment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::SplitCapacityCommitment {
        super::builder::reservation_service::SplitCapacityCommitment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Merges capacity commitments of the same plan into a single commitment.
    ///
    /// The resulting capacity commitment has the greater commitment_end_time
    /// out of the to-be-merged capacity commitments.
    ///
    /// Attempting to merge capacity commitments of different plan will fail
    /// with the error code `google.rpc.Code.FAILED_PRECONDITION`.
    pub fn merge_capacity_commitments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::MergeCapacityCommitments {
        super::builder::reservation_service::MergeCapacityCommitments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates an assignment object which allows the given project to submit jobs
    /// of a certain type using slots from the specified reservation.
    ///
    /// Currently a
    /// resource (project, folder, organization) can only have one assignment per
    /// each (job_type, location) combination, and that reservation will be used
    /// for all jobs of the matching type.
    ///
    /// Different assignments can be created on different levels of the
    /// projects, folders or organization hierarchy.  During query execution,
    /// the assignment is looked up at the project, folder and organization levels
    /// in that order. The first assignment found is applied to the query.
    ///
    /// When creating assignments, it does not matter if other assignments exist at
    /// higher levels.
    ///
    /// Example:
    ///
    /// * The organization `organizationA` contains two projects, `project1`
    ///   and `project2`.
    /// * Assignments for all three entities (`organizationA`, `project1`, and
    ///   `project2`) could all be created and mapped to the same or different
    ///   reservations.
    ///
    /// "None" assignments represent an absence of the assignment. Projects
    /// assigned to None use on-demand pricing. To create a "None" assignment, use
    /// "none" as a reservation_id in the parent. Example parent:
    /// `projects/myproject/locations/US/reservations/none`.
    ///
    /// Returns `google.rpc.Code.PERMISSION_DENIED` if user does not have
    /// 'bigquery.admin' permissions on the project using the reservation
    /// and the project that owns this reservation.
    ///
    /// Returns `google.rpc.Code.INVALID_ARGUMENT` when location of the assignment
    /// does not match location of the reservation.
    pub fn create_assignment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::CreateAssignment {
        super::builder::reservation_service::CreateAssignment::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists assignments.
    ///
    /// Only explicitly created assignments will be returned.
    ///
    /// Example:
    ///
    /// * Organization `organizationA` contains two projects, `project1` and
    ///   `project2`.
    /// * Reservation `res1` exists and was created previously.
    /// * CreateAssignment was used previously to define the following
    ///   associations between entities and reservations: `<organizationA, res1>`
    ///   and `<project1, res1>`
    ///
    /// In this example, ListAssignments will just return the above two assignments
    /// for reservation `res1`, and no expansion/merge will happen.
    ///
    /// The wildcard "-" can be used for
    /// reservations in the request. In that case all assignments belongs to the
    /// specified project and location will be listed.
    ///
    /// **Note** "-" cannot be used for projects nor locations.
    pub fn list_assignments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::ListAssignments {
        super::builder::reservation_service::ListAssignments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a assignment. No expansion will happen.
    ///
    /// Example:
    ///
    /// * Organization `organizationA` contains two projects, `project1` and
    ///   `project2`.
    /// * Reservation `res1` exists and was created previously.
    /// * CreateAssignment was used previously to define the following
    ///   associations between entities and reservations: `<organizationA, res1>`
    ///   and `<project1, res1>`
    ///
    /// In this example, deletion of the `<organizationA, res1>` assignment won't
    /// affect the other assignment `<project1, res1>`. After said deletion,
    /// queries from `project1` will still use `res1` while queries from
    /// `project2` will switch to use on-demand mode.
    pub fn delete_assignment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::DeleteAssignment {
        super::builder::reservation_service::DeleteAssignment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deprecated: Looks up assignments for a specified resource for a particular
    /// region. If the request is about a project:
    ///
    /// . Assignments created on the project will be returned if they exist.
    /// . Otherwise assignments created on the closest ancestor will be
    ///   returned.
    /// . Assignments for different JobTypes will all be returned.
    ///
    /// The same logic applies if the request is about a folder.
    ///
    /// If the request is about an organization, then assignments created on the
    /// organization will be returned (organization doesn't have ancestors).
    ///
    /// Comparing to ListAssignments, there are some behavior
    /// differences:
    ///
    /// . permission on the assignee will be verified in this API.
    /// . Hierarchy lookup (project->folder->organization) happens in this API.
    /// . Parent here is `projects/*/locations/*`, instead of
    ///   `projects/*/locations/*reservations/*`.
    ///
    /// **Note** "-" cannot be used for projects
    /// nor locations.
    pub fn search_assignments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::SearchAssignments {
        super::builder::reservation_service::SearchAssignments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Looks up assignments for a specified resource for a particular region.
    /// If the request is about a project:
    ///
    /// . Assignments created on the project will be returned if they exist.
    /// . Otherwise assignments created on the closest ancestor will be
    ///   returned.
    /// . Assignments for different JobTypes will all be returned.
    ///
    /// The same logic applies if the request is about a folder.
    ///
    /// If the request is about an organization, then assignments created on the
    /// organization will be returned (organization doesn't have ancestors).
    ///
    /// Comparing to ListAssignments, there are some behavior
    /// differences:
    ///
    /// . permission on the assignee will be verified in this API.
    /// . Hierarchy lookup (project->folder->organization) happens in this API.
    /// . Parent here is `projects/*/locations/*`, instead of
    ///   `projects/*/locations/*reservations/*`.
    pub fn search_all_assignments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::SearchAllAssignments {
        super::builder::reservation_service::SearchAllAssignments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Moves an assignment under a new reservation.
    ///
    /// This differs from removing an existing assignment and recreating a new one
    /// by providing a transactional change that ensures an assignee always has an
    /// associated reservation.
    pub fn move_assignment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::MoveAssignment {
        super::builder::reservation_service::MoveAssignment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates an existing assignment.
    ///
    /// Only the `priority` field can be updated.
    pub fn update_assignment(
        &self,
        assignment: impl Into<crate::model::Assignment>,
    ) -> super::builder::reservation_service::UpdateAssignment {
        super::builder::reservation_service::UpdateAssignment::new(self.inner.clone())
            .set_assignment(assignment.into())
    }

    /// Retrieves a BI reservation.
    pub fn get_bi_reservation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::reservation_service::GetBiReservation {
        super::builder::reservation_service::GetBiReservation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates a BI reservation.
    ///
    /// Only fields specified in the `field_mask` are updated.
    ///
    /// A singleton BI reservation always exists with default size 0.
    /// In order to reserve BI capacity it needs to be updated to an amount
    /// greater than 0. In order to release BI capacity reservation size
    /// must be set to 0.
    pub fn update_bi_reservation(
        &self,
        bi_reservation: impl Into<crate::model::BiReservation>,
    ) -> super::builder::reservation_service::UpdateBiReservation {
        super::builder::reservation_service::UpdateBiReservation::new(self.inner.clone())
            .set_bi_reservation(bi_reservation.into())
    }
}
