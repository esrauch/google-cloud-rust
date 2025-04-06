// Copyright 2024 Google LLC
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

use google_cloud_wkt as wkt;
use serde_json::json;
type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Helper {
    pub field_double: Option<wkt::DoubleValue>,
    pub field_float: Option<wkt::FloatValue>,
}

#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Repeated {
    pub field_double: Vec<wkt::DoubleValue>,
    pub field_float: Vec<wkt::FloatValue>,
}

#[test]
fn serialize_in_struct() -> Result {
    let input = Helper {
        field_double: Some(f64::INFINITY),
        field_float: Some(f32::NAN),
    };
    let json = serde_json::to_value(&input)?;
    let want = json!({
        "fieldDouble": null,  // Data loss of the Infinity value
        "fieldFloat":  null,  // Data loss of the NaN value
    });
    assert_eq!(json, want);

    let roundtrip = serde_json::from_value::<Helper>(json)?;
    // Data does not round trip, instead the fields are unset.
    assert_ne!(input, roundtrip);
    assert_eq!(roundtrip, Helper {
        field_double: None,
        field_float: None
    });
    Ok(())
}

#[test]
fn serialize_in_repeated() -> Result {
    let input = Repeated {
        field_double: vec![42.0_f64, f64::INFINITY, 43f64],
        field_float: vec![42.0_f32, f32::NAN, 43f32],
    };
    let json = serde_json::to_value(&input)?;

    // The serialize state emits null for Infinity and NaN values
    let want = json!({
        "fieldDouble":  [42_f64, null, 43f64],
        "fieldFloat":   [42_f32, null, 43f32],
    });
    assert_eq!(json, want);

    let roundtrip = serde_json::from_value::<Repeated>(json);

    // The data that it serializes is a parse failure when it tries to parse back.
    // (It also would be parse failure if that serialized state were sent to a conformant server)
    assert!(roundtrip.is_err()); 
    Ok(())
}
