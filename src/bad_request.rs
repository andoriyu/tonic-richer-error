use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use super::{pb, FromAny, IntoAny};

#[derive(Clone, Debug)]
pub struct FieldViolation {
    pub field: String,
    pub description: String,
}
#[derive(Clone, Debug)]
pub struct BadRequest {
    pub field_violations: Vec<FieldViolation>,
}

impl BadRequest {
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.BadRequest";

    pub fn empty() -> Self {
        BadRequest {
            field_violations: Vec::new(),
        }
    }

    pub fn add_violation(
        &mut self,
        field: impl Into<String>,
        violation: impl Into<String>,
    ) -> &mut Self {
        self.field_violations.append(&mut vec![FieldViolation {
            field: field.into(),
            description: violation.into(),
        }]);
        self
    }

    pub fn with_violation(field: impl Into<String>, violation: impl Into<String>) -> Self {
        BadRequest {
            field_violations: vec![FieldViolation {
                field: field.into(),
                description: violation.into(),
            }],
        }
    }

    pub fn has_violations(&self) -> bool {
        self.field_violations.is_empty() == false
    }
}

impl IntoAny for BadRequest {
    fn into_any(self) -> Result<Any, EncodeError> {
        let detail_data = pb::BadRequest {
            field_violations: self
                .field_violations
                .into_iter()
                .map(|v| pb::bad_request::FieldViolation {
                    field: v.field,
                    description: v.description,
                })
                .collect(),
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(detail_data.encoded_len());
        detail_data.encode(&mut buf)?;

        Ok(Any {
            type_url: BadRequest::TYPE_URL.to_string(),
            value: buf,
        })
    }
}

impl FromAny for BadRequest {
    fn from_any(any: Any) -> Result<Self, DecodeError> {
        let buf: &[u8] = &any.value;
        let bad_req = pb::BadRequest::decode(buf)?;

        let bad_req = BadRequest {
            field_violations: bad_req
                .field_violations
                .into_iter()
                .map(|v| FieldViolation {
                    field: v.field,
                    description: v.description,
                })
                .collect(),
        };

        Ok(bad_req)
    }
}

#[cfg(test)]
mod tests {

    use crate::{FromAny, IntoAny};

    use super::BadRequest;

    #[test]
    fn gen_bad_request() {
        let mut br_details = BadRequest::empty();
        let formatted = format!("{:?}", br_details);

        println!("empty BadRequest -> {formatted}");

        let expected = "BadRequest { field_violations: [] }";

        assert!(
            formatted.eq(expected),
            "empty BadRequest differs from expected result"
        );

        assert!(
            br_details.has_violations() == false,
            "empty BadRequest returns 'true' from .has_violations()"
        );

        br_details
            .add_violation("field_a", "description_a")
            .add_violation("field_b", "description_b");

        let formatted = format!("{:?}", br_details);

        println!("filled BadRequest -> {formatted}");

        let expected_filled = "BadRequest { field_violations: [FieldViolation { field: \"field_a\", description: \"description_a\" }, FieldViolation { field: \"field_b\", description: \"description_b\" }] }";

        assert!(
            formatted.eq(expected_filled),
            "filled BadRequest differs from expected result"
        );

        assert!(
            br_details.has_violations() == true,
            "filled BadRequest returns 'false' from .has_violations()"
        );

        let gen_any = match br_details.into_any() {
            Err(error) => panic!("Error generating Any from BadRequest: {:?}", error),
            Ok(gen_any) => gen_any,
        };
        let formatted = format!("{:?}", gen_any);

        println!("Any generated from BadRequest -> {formatted}");

        let expected = "Any { type_url: \"type.googleapis.com/google.rpc.BadRequest\", value: [10, 24, 10, 7, 102, 105, 101, 108, 100, 95, 97, 18, 13, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 95, 97, 10, 24, 10, 7, 102, 105, 101, 108, 100, 95, 98, 18, 13, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 95, 98] }";

        assert!(
            formatted.eq(expected),
            "Any from filled BadRequest differs from expected result"
        );

        let br_details = match BadRequest::from_any(gen_any) {
            Err(error) => panic!("Error generating BadRequest from Any: {:?}", error),
            Ok(from_any) => from_any,
        };

        let formatted = format!("{:?}", br_details);

        println!("BadRequest generated from Any -> {formatted}");

        assert!(
            formatted.eq(expected_filled),
            "BadRequest from Any differs from expected result"
        );
    }
}
