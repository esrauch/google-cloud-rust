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

// ignore_for_file: unused_import

/// The Google Cloud client for the Cloud Metadata API.
///
/// This API provides static metadata about Google Cloud Platform. Currently,
/// it only provides basic information about Google Cloud locations, such as
/// zones, regions, and countries.
library;

import 'package:google_cloud_gax/common.dart';
import 'package:google_cloud_gax/src/json_helpers.dart';
import 'package:google_cloud_protobuf/protobuf.dart';
import 'package:http/http.dart' as http;

/// An abstract interface that provides location-related information for
/// a service. Service-specific metadata is provided through the
/// `Location.metadata` field.
class Locations {
  static const String _host = 'cloud.googleapis.com';

  final ServiceClient _client;

  Locations({required http.Client client})
      : _client = ServiceClient(client: client);

  /// Lists information about the supported locations for this service.
  Future<ListLocationsResponse> listLocations(
      ListLocationsRequest request) async {
    final url = Uri.https(_host, '/v1/${request.name}');
    final response = await _client.get(url);
    return ListLocationsResponse.fromJson(response);
  }

  /// Gets information about a location.
  Future<Location> getLocation(GetLocationRequest request) async {
    final url = Uri.https(_host, '/v1/${request.name}');
    final response = await _client.get(url);
    return Location.fromJson(response);
  }

  /// Closes the client and cleans up any resources associated with it.
  ///
  /// Once [close] is called, no other methods should be called.
  void close() => _client.close();
}

/// The request message for `Locations.ListLocations`.
class ListLocationsRequest extends Message {
  static const String fullyQualifiedName =
      'google.cloud.location.ListLocationsRequest';

  /// The resource that owns the locations collection, if applicable.
  final String name;

  /// The standard list filter.
  final String? filter;

  /// The standard list page size.
  final int? pageSize;

  /// The standard list page token.
  final String? pageToken;

  ListLocationsRequest({
    required this.name,
    this.filter,
    this.pageSize,
    this.pageToken,
  }) : super(fullyQualifiedName);

  factory ListLocationsRequest.fromJson(Map<String, dynamic> json) {
    return ListLocationsRequest(
      name: json['name'],
      filter: json['filter'],
      pageSize: json['pageSize'],
      pageToken: json['pageToken'],
    );
  }

  @override
  Object toJson() {
    return {
      'name': name,
      if (filter != null) 'filter': filter,
      if (pageSize != null) 'pageSize': pageSize,
      if (pageToken != null) 'pageToken': pageToken,
    };
  }

  @override
  String toString() {
    final contents = [
      'name=$name',
      if (filter != null) 'filter=$filter',
      if (pageSize != null) 'pageSize=$pageSize',
      if (pageToken != null) 'pageToken=$pageToken',
    ].join(',');
    return 'ListLocationsRequest($contents)';
  }
}

/// The response message for `Locations.ListLocations`.
class ListLocationsResponse extends Message {
  static const String fullyQualifiedName =
      'google.cloud.location.ListLocationsResponse';

  /// A list of locations that matches the specified filter in the request.
  final List<Location>? locations;

  /// The standard List next-page token.
  final String? nextPageToken;

  ListLocationsResponse({
    this.locations,
    this.nextPageToken,
  }) : super(fullyQualifiedName);

  factory ListLocationsResponse.fromJson(Map<String, dynamic> json) {
    return ListLocationsResponse(
      locations: decodeList(json['locations'], Location.fromJson),
      nextPageToken: json['nextPageToken'],
    );
  }

  @override
  Object toJson() {
    return {
      if (locations != null) 'locations': encodeList(locations),
      if (nextPageToken != null) 'nextPageToken': nextPageToken,
    };
  }

  @override
  String toString() {
    final contents = [
      if (nextPageToken != null) 'nextPageToken=$nextPageToken',
    ].join(',');
    return 'ListLocationsResponse($contents)';
  }
}

/// The request message for `Locations.GetLocation`.
class GetLocationRequest extends Message {
  static const String fullyQualifiedName =
      'google.cloud.location.GetLocationRequest';

  /// Resource name for the location.
  final String name;

  GetLocationRequest({
    required this.name,
  }) : super(fullyQualifiedName);

  factory GetLocationRequest.fromJson(Map<String, dynamic> json) {
    return GetLocationRequest(
      name: json['name'],
    );
  }

  @override
  Object toJson() {
    return {
      'name': name,
    };
  }

  @override
  String toString() {
    final contents = [
      'name=$name',
    ].join(',');
    return 'GetLocationRequest($contents)';
  }
}

/// A resource that represents Google Cloud Platform location.
class Location extends Message {
  static const String fullyQualifiedName = 'google.cloud.location.Location';

  /// Resource name for the location, which may vary between implementations.
  /// For example: `"projects/example-project/locations/us-east1"`
  final String? name;

  /// The canonical id for this location. For example: `"us-east1"`.
  final String? locationId;

  /// The friendly name for this location, typically a nearby city name.
  /// For example, "Tokyo".
  final String? displayName;

  /// Cross-service attributes for the location. For example
  ///
  ///     {"cloud.googleapis.com/region": "us-east1"}
  final Map<String, String>? labels;

  /// Service-specific metadata. For example the available capacity at the given
  /// location.
  final Any? metadata;

  Location({
    this.name,
    this.locationId,
    this.displayName,
    this.labels,
    this.metadata,
  }) : super(fullyQualifiedName);

  factory Location.fromJson(Map<String, dynamic> json) {
    return Location(
      name: json['name'],
      locationId: json['locationId'],
      displayName: json['displayName'],
      labels: (json['labels'] as Map?)?.cast(),
      metadata: decode(json['metadata'], Any.fromJson),
    );
  }

  @override
  Object toJson() {
    return {
      if (name != null) 'name': name,
      if (locationId != null) 'locationId': locationId,
      if (displayName != null) 'displayName': displayName,
      if (labels != null) 'labels': labels,
      if (metadata != null) 'metadata': metadata!.toJson(),
    };
  }

  @override
  String toString() {
    final contents = [
      if (name != null) 'name=$name',
      if (locationId != null) 'locationId=$locationId',
      if (displayName != null) 'displayName=$displayName',
    ].join(',');
    return 'Location($contents)';
  }
}
