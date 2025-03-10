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
#![no_implicit_prelude]
extern crate async_trait;
extern crate bytes;
extern crate gax;
extern crate lazy_static;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// A notification object for notifying customers about security and privacy
/// issues.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Notification {
    /// The resource name of the notification.
    /// Format:
    /// organizations/{organization}/locations/{location}/notifications/{notification}
    /// or projects/{project}/locations/{location}/notifications/{notification}.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The subject line of the notification.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub subject: std::option::Option<crate::model::Subject>,

    /// A list of messages in the notification.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub messages: std::vec::Vec<crate::model::Message>,

    /// Output only. Time the notification was created.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,

    /// Type of notification
    pub notification_type: crate::model::NotificationType,
}

impl Notification {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::Notification::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [subject][crate::model::Notification::subject].
    pub fn set_subject<T: std::convert::Into<std::option::Option<crate::model::Subject>>>(
        mut self,
        v: T,
    ) -> Self {
        self.subject = v.into();
        self
    }

    /// Sets the value of [create_time][crate::model::Notification::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [notification_type][crate::model::Notification::notification_type].
    pub fn set_notification_type<T: std::convert::Into<crate::model::NotificationType>>(
        mut self,
        v: T,
    ) -> Self {
        self.notification_type = v.into();
        self
    }

    /// Sets the value of [messages][crate::model::Notification::messages].
    pub fn set_messages<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Message>,
    {
        use std::iter::Iterator;
        self.messages = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for Notification {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.Notification"
    }
}

/// A text object containing the English text and its localized copies.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Text {
    /// The English copy.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub en_text: std::string::String,

    /// The requested localized copy (if applicable).
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub localized_text: std::string::String,

    /// Status of the localization.
    pub localization_state: crate::model::LocalizationState,
}

impl Text {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [en_text][crate::model::Text::en_text].
    pub fn set_en_text<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.en_text = v.into();
        self
    }

    /// Sets the value of [localized_text][crate::model::Text::localized_text].
    pub fn set_localized_text<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.localized_text = v.into();
        self
    }

    /// Sets the value of [localization_state][crate::model::Text::localization_state].
    pub fn set_localization_state<T: std::convert::Into<crate::model::LocalizationState>>(
        mut self,
        v: T,
    ) -> Self {
        self.localization_state = v.into();
        self
    }
}

impl wkt::message::Message for Text {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.Text"
    }
}

/// A subject line of a notification.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Subject {
    /// The text content.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub text: std::option::Option<crate::model::Text>,
}

impl Subject {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [text][crate::model::Subject::text].
    pub fn set_text<T: std::convert::Into<std::option::Option<crate::model::Text>>>(
        mut self,
        v: T,
    ) -> Self {
        self.text = v.into();
        self
    }
}

impl wkt::message::Message for Subject {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.Subject"
    }
}

/// A message which contains notification details.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Message {
    /// The message content.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub body: std::option::Option<crate::model::message::Body>,

    /// The attachments to download.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub attachments: std::vec::Vec<crate::model::Attachment>,

    /// The Message creation timestamp.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,

    /// Time when Message was localized
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub localization_time: std::option::Option<wkt::Timestamp>,
}

impl Message {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [body][crate::model::Message::body].
    pub fn set_body<T: std::convert::Into<std::option::Option<crate::model::message::Body>>>(
        mut self,
        v: T,
    ) -> Self {
        self.body = v.into();
        self
    }

    /// Sets the value of [create_time][crate::model::Message::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [localization_time][crate::model::Message::localization_time].
    pub fn set_localization_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.localization_time = v.into();
        self
    }

    /// Sets the value of [attachments][crate::model::Message::attachments].
    pub fn set_attachments<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Attachment>,
    {
        use std::iter::Iterator;
        self.attachments = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for Message {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.Message"
    }
}

/// Defines additional types related to Message
pub mod message {
    #[allow(unused_imports)]
    use super::*;

    /// A message body containing text.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct Body {
        /// The text content of the message body.
        #[serde(skip_serializing_if = "std::option::Option::is_none")]
        pub text: std::option::Option<crate::model::Text>,
    }

    impl Body {
        pub fn new() -> Self {
            std::default::Default::default()
        }

        /// Sets the value of [text][crate::model::message::Body::text].
        pub fn set_text<T: std::convert::Into<std::option::Option<crate::model::Text>>>(
            mut self,
            v: T,
        ) -> Self {
            self.text = v.into();
            self
        }
    }

    impl wkt::message::Message for Body {
        fn typename() -> &'static str {
            "type.googleapis.com/google.cloud.advisorynotifications.v1.Message.Body"
        }
    }
}

/// Attachment with specific information about the issue.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Attachment {
    /// The title of the attachment.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub display_name: std::string::String,

    /// Data type of the attachment.
    #[serde(flatten, skip_serializing_if = "std::option::Option::is_none")]
    pub data: std::option::Option<crate::model::attachment::Data>,
}

impl Attachment {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [display_name][crate::model::Attachment::display_name].
    pub fn set_display_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.display_name = v.into();
        self
    }

    /// Sets the value of `data`.
    pub fn set_data<T: std::convert::Into<std::option::Option<crate::model::attachment::Data>>>(
        mut self,
        v: T,
    ) -> Self {
        self.data = v.into();
        self
    }

    /// The value of [data][crate::model::Attachment::data]
    /// if it holds a `Csv`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_csv(&self) -> std::option::Option<&std::boxed::Box<crate::model::Csv>> {
        #[allow(unreachable_patterns)]
        self.data.as_ref().and_then(|v| match v {
            crate::model::attachment::Data::Csv(v) => std::option::Option::Some(v),
            _ => std::option::Option::None,
        })
    }

    /// Sets the value of [data][crate::model::Attachment::data]
    /// to hold a `Csv`.
    ///
    /// Note that all the setters affecting `data` are
    /// mutually exclusive.
    pub fn set_csv<T: std::convert::Into<std::boxed::Box<crate::model::Csv>>>(
        mut self,
        v: T,
    ) -> Self {
        self.data = std::option::Option::Some(crate::model::attachment::Data::Csv(v.into()));
        self
    }
}

impl wkt::message::Message for Attachment {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.Attachment"
    }
}

/// Defines additional types related to Attachment
pub mod attachment {
    #[allow(unused_imports)]
    use super::*;

    /// Data type of the attachment.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub enum Data {
        /// A CSV file attachment. Max size is 10 MB.
        Csv(std::boxed::Box<crate::model::Csv>),
    }
}

/// A representation of a CSV file attachment, as a list of column headers and
/// a list of data rows.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Csv {
    /// The list of headers for data columns in a CSV file.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub headers: std::vec::Vec<std::string::String>,

    /// The list of data rows in a CSV file, as string arrays rather than as a
    /// single comma-separated string.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub data_rows: std::vec::Vec<crate::model::csv::CsvRow>,
}

impl Csv {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [headers][crate::model::Csv::headers].
    pub fn set_headers<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.headers = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of [data_rows][crate::model::Csv::data_rows].
    pub fn set_data_rows<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::csv::CsvRow>,
    {
        use std::iter::Iterator;
        self.data_rows = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for Csv {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.Csv"
    }
}

/// Defines additional types related to Csv
pub mod csv {
    #[allow(unused_imports)]
    use super::*;

    /// A representation of a single data row in a CSV file.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct CsvRow {
        /// The data entries in a CSV file row, as a string array rather than a
        /// single comma-separated string.
        #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
        pub entries: std::vec::Vec<std::string::String>,
    }

    impl CsvRow {
        pub fn new() -> Self {
            std::default::Default::default()
        }

        /// Sets the value of [entries][crate::model::csv::CsvRow::entries].
        pub fn set_entries<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.entries = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl wkt::message::Message for CsvRow {
        fn typename() -> &'static str {
            "type.googleapis.com/google.cloud.advisorynotifications.v1.Csv.CsvRow"
        }
    }
}

/// Request for fetching all notifications for a given parent.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListNotificationsRequest {
    /// Required. The parent, which owns this collection of notifications.
    /// Must be of the form "organizations/{organization}/locations/{location}"
    /// or "projects/{project}/locations/{location}".
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// The maximum number of notifications to return. The service may return
    /// fewer than this value. If unspecified or equal to 0, at most 50
    /// notifications will be returned. The maximum value is 50; values above 50
    /// will be coerced to 50.
    pub page_size: i32,

    /// A page token returned from a previous request.
    /// When paginating, all other parameters provided in the request
    /// must match the call that returned the page token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,

    /// Specifies which parts of the notification resource should be returned
    /// in the response.
    pub view: crate::model::NotificationView,

    /// ISO code for requested localization language.  If unset, will be
    /// interpereted as "en". If the requested language is valid, but not supported
    /// for this notification, English will be returned with an "Not applicable"
    /// LocalizationState. If the ISO code is invalid (i.e. not a real language),
    /// this RPC will throw an error.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub language_code: std::string::String,
}

impl ListNotificationsRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::ListNotificationsRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListNotificationsRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListNotificationsRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of [view][crate::model::ListNotificationsRequest::view].
    pub fn set_view<T: std::convert::Into<crate::model::NotificationView>>(mut self, v: T) -> Self {
        self.view = v.into();
        self
    }

    /// Sets the value of [language_code][crate::model::ListNotificationsRequest::language_code].
    pub fn set_language_code<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.language_code = v.into();
        self
    }
}

impl wkt::message::Message for ListNotificationsRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.ListNotificationsRequest"
    }
}

/// Response of ListNotifications endpoint.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListNotificationsResponse {
    /// List of notifications under a given parent.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub notifications: std::vec::Vec<crate::model::Notification>,

    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,

    /// Estimation of a total number of notifications.
    pub total_size: i32,
}

impl ListNotificationsResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::ListNotificationsResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [total_size][crate::model::ListNotificationsResponse::total_size].
    pub fn set_total_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.total_size = v.into();
        self
    }

    /// Sets the value of [notifications][crate::model::ListNotificationsResponse::notifications].
    pub fn set_notifications<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Notification>,
    {
        use std::iter::Iterator;
        self.notifications = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListNotificationsResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.ListNotificationsResponse"
    }
}

#[cfg(feature = "unstable-stream")]
impl gax::paginator::PageableResponse for ListNotificationsResponse {
    type PageItem = crate::model::Notification;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.notifications
    }

    fn next_page_token(&self) -> std::string::String {
        gax::paginator::extract_token(&self.next_page_token)
    }
}

/// Request for fetching a notification.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetNotificationRequest {
    /// Required. A name of the notification to retrieve.
    /// Format:
    /// organizations/{organization}/locations/{location}/notifications/{notification}
    /// or projects/{projects}/locations/{location}/notifications/{notification}.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// ISO code for requested localization language. If unset, will be
    /// interpereted as "en". If the requested language is valid, but not supported
    /// for this notification, English will be returned with an "Not applicable"
    /// LocalizationState. If the ISO code is invalid (i.e. not a real language),
    /// this RPC will throw an error.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub language_code: std::string::String,
}

impl GetNotificationRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetNotificationRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [language_code][crate::model::GetNotificationRequest::language_code].
    pub fn set_language_code<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.language_code = v.into();
        self
    }
}

impl wkt::message::Message for GetNotificationRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.GetNotificationRequest"
    }
}

/// Settings for Advisory Notifications.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Settings {
    /// Identifier. The resource name of the settings to retrieve.
    /// Format:
    /// organizations/{organization}/locations/{location}/settings or
    /// projects/{projects}/locations/{location}/settings.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Required. Map of each notification type and its settings to get/set all
    /// settings at once. The server will validate the value for each notification
    /// type.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub notification_settings:
        std::collections::HashMap<std::string::String, crate::model::NotificationSettings>,

    /// Required. Fingerprint for optimistic concurrency returned in Get requests.
    /// Must be provided for Update requests. If the value provided does not match
    /// the value known to the server, ABORTED will be thrown, and the client
    /// should retry the read-modify-write cycle.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub etag: std::string::String,
}

impl Settings {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::Settings::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [etag][crate::model::Settings::etag].
    pub fn set_etag<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }

    /// Sets the value of [notification_settings][crate::model::Settings::notification_settings].
    pub fn set_notification_settings<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<crate::model::NotificationSettings>,
    {
        use std::iter::Iterator;
        self.notification_settings = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }
}

impl wkt::message::Message for Settings {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.Settings"
    }
}

/// Settings for each NotificationType.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct NotificationSettings {
    /// Whether the associated NotificationType is enabled.
    pub enabled: bool,
}

impl NotificationSettings {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [enabled][crate::model::NotificationSettings::enabled].
    pub fn set_enabled<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.enabled = v.into();
        self
    }
}

impl wkt::message::Message for NotificationSettings {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.NotificationSettings"
    }
}

/// Request of GetSettings endpoint.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetSettingsRequest {
    /// Required. The resource name of the settings to retrieve.
    /// Format:
    /// organizations/{organization}/locations/{location}/settings or
    /// projects/{projects}/locations/{location}/settings.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl GetSettingsRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetSettingsRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetSettingsRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.GetSettingsRequest"
    }
}

/// Request of UpdateSettings endpoint.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct UpdateSettingsRequest {
    /// Required. New settings.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub settings: std::option::Option<crate::model::Settings>,
}

impl UpdateSettingsRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [settings][crate::model::UpdateSettingsRequest::settings].
    pub fn set_settings<T: std::convert::Into<std::option::Option<crate::model::Settings>>>(
        mut self,
        v: T,
    ) -> Self {
        self.settings = v.into();
        self
    }
}

impl wkt::message::Message for UpdateSettingsRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.advisorynotifications.v1.UpdateSettingsRequest"
    }
}

/// Notification view.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NotificationView(i32);

impl NotificationView {
    /// Not specified, equivalent to BASIC.
    pub const NOTIFICATION_VIEW_UNSPECIFIED: NotificationView = NotificationView::new(0);

    /// Server responses only include title, creation time and Notification ID.
    /// Note: for internal use responses also include the last update time,
    /// the latest message text and whether notification has attachments.
    pub const BASIC: NotificationView = NotificationView::new(1);

    /// Include everything.
    pub const FULL: NotificationView = NotificationView::new(2);

    /// Creates a new NotificationView instance.
    pub(crate) const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Gets the enum value.
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Gets the enum value as a string.
    pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
        match self.0 {
            0 => std::borrow::Cow::Borrowed("NOTIFICATION_VIEW_UNSPECIFIED"),
            1 => std::borrow::Cow::Borrowed("BASIC"),
            2 => std::borrow::Cow::Borrowed("FULL"),
            _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
        }
    }

    /// Creates an enum value from the value name.
    pub fn from_str_name(name: &str) -> std::option::Option<Self> {
        match name {
            "NOTIFICATION_VIEW_UNSPECIFIED" => {
                std::option::Option::Some(Self::NOTIFICATION_VIEW_UNSPECIFIED)
            }
            "BASIC" => std::option::Option::Some(Self::BASIC),
            "FULL" => std::option::Option::Some(Self::FULL),
            _ => std::option::Option::None,
        }
    }
}

impl std::convert::From<i32> for NotificationView {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl std::default::Default for NotificationView {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Status of localized text.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LocalizationState(i32);

impl LocalizationState {
    /// Not used.
    pub const LOCALIZATION_STATE_UNSPECIFIED: LocalizationState = LocalizationState::new(0);

    /// Localization is not applicable for requested language. This can happen
    /// when:
    ///
    /// - The requested language was not supported by Advisory Notifications at the
    ///   time of localization (including notifications created before the
    ///   localization feature was launched).
    /// - The requested language is English, so only the English text is returned.
    pub const LOCALIZATION_STATE_NOT_APPLICABLE: LocalizationState = LocalizationState::new(1);

    /// Localization for requested language is in progress, and not ready yet.
    pub const LOCALIZATION_STATE_PENDING: LocalizationState = LocalizationState::new(2);

    /// Localization for requested language is completed.
    pub const LOCALIZATION_STATE_COMPLETED: LocalizationState = LocalizationState::new(3);

    /// Creates a new LocalizationState instance.
    pub(crate) const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Gets the enum value.
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Gets the enum value as a string.
    pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
        match self.0 {
            0 => std::borrow::Cow::Borrowed("LOCALIZATION_STATE_UNSPECIFIED"),
            1 => std::borrow::Cow::Borrowed("LOCALIZATION_STATE_NOT_APPLICABLE"),
            2 => std::borrow::Cow::Borrowed("LOCALIZATION_STATE_PENDING"),
            3 => std::borrow::Cow::Borrowed("LOCALIZATION_STATE_COMPLETED"),
            _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
        }
    }

    /// Creates an enum value from the value name.
    pub fn from_str_name(name: &str) -> std::option::Option<Self> {
        match name {
            "LOCALIZATION_STATE_UNSPECIFIED" => {
                std::option::Option::Some(Self::LOCALIZATION_STATE_UNSPECIFIED)
            }
            "LOCALIZATION_STATE_NOT_APPLICABLE" => {
                std::option::Option::Some(Self::LOCALIZATION_STATE_NOT_APPLICABLE)
            }
            "LOCALIZATION_STATE_PENDING" => {
                std::option::Option::Some(Self::LOCALIZATION_STATE_PENDING)
            }
            "LOCALIZATION_STATE_COMPLETED" => {
                std::option::Option::Some(Self::LOCALIZATION_STATE_COMPLETED)
            }
            _ => std::option::Option::None,
        }
    }
}

impl std::convert::From<i32> for LocalizationState {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl std::default::Default for LocalizationState {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Type of notification
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NotificationType(i32);

impl NotificationType {
    /// Default type
    pub const NOTIFICATION_TYPE_UNSPECIFIED: NotificationType = NotificationType::new(0);

    /// Security and privacy advisory notifications
    pub const NOTIFICATION_TYPE_SECURITY_PRIVACY_ADVISORY: NotificationType =
        NotificationType::new(1);

    /// Sensitive action notifications
    pub const NOTIFICATION_TYPE_SENSITIVE_ACTIONS: NotificationType = NotificationType::new(2);

    /// General security MSA
    pub const NOTIFICATION_TYPE_SECURITY_MSA: NotificationType = NotificationType::new(3);

    /// Threat horizons MSA
    pub const NOTIFICATION_TYPE_THREAT_HORIZONS: NotificationType = NotificationType::new(4);

    /// Creates a new NotificationType instance.
    pub(crate) const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Gets the enum value.
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Gets the enum value as a string.
    pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
        match self.0 {
            0 => std::borrow::Cow::Borrowed("NOTIFICATION_TYPE_UNSPECIFIED"),
            1 => std::borrow::Cow::Borrowed("NOTIFICATION_TYPE_SECURITY_PRIVACY_ADVISORY"),
            2 => std::borrow::Cow::Borrowed("NOTIFICATION_TYPE_SENSITIVE_ACTIONS"),
            3 => std::borrow::Cow::Borrowed("NOTIFICATION_TYPE_SECURITY_MSA"),
            4 => std::borrow::Cow::Borrowed("NOTIFICATION_TYPE_THREAT_HORIZONS"),
            _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
        }
    }

    /// Creates an enum value from the value name.
    pub fn from_str_name(name: &str) -> std::option::Option<Self> {
        match name {
            "NOTIFICATION_TYPE_UNSPECIFIED" => {
                std::option::Option::Some(Self::NOTIFICATION_TYPE_UNSPECIFIED)
            }
            "NOTIFICATION_TYPE_SECURITY_PRIVACY_ADVISORY" => {
                std::option::Option::Some(Self::NOTIFICATION_TYPE_SECURITY_PRIVACY_ADVISORY)
            }
            "NOTIFICATION_TYPE_SENSITIVE_ACTIONS" => {
                std::option::Option::Some(Self::NOTIFICATION_TYPE_SENSITIVE_ACTIONS)
            }
            "NOTIFICATION_TYPE_SECURITY_MSA" => {
                std::option::Option::Some(Self::NOTIFICATION_TYPE_SECURITY_MSA)
            }
            "NOTIFICATION_TYPE_THREAT_HORIZONS" => {
                std::option::Option::Some(Self::NOTIFICATION_TYPE_THREAT_HORIZONS)
            }
            _ => std::option::Option::None,
        }
    }
}

impl std::convert::From<i32> for NotificationType {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl std::default::Default for NotificationType {
    fn default() -> Self {
        Self::new(0)
    }
}
