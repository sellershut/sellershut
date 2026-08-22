use sqlx::prelude::Type;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{custom_url::Url, redacted_secret::RedactedSecret};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub ap_id: Url,
    pub avatar: Option<String>,
    pub name: Option<String>,
    pub inbox: Url,
    pub public_key: String,
    pub kind: ActorType,
    #[serde(serialize_with = "serialize_redacted_secret")]
    pub private_key: Option<RedactedSecret>,
    pub created_at: OffsetDateTime,
    pub last_refreshed_at: OffsetDateTime,
    pub is_local: bool,
}

use serde::Serializer;

fn serialize_redacted_secret<S>(
    s: &Option<RedactedSecret>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match s {
        Some(v) => serializer.serialize_str(&v.expose()),
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_kind")]
#[sqlx(rename_all = "PascalCase")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum ActorType {
    Person,
    Service,
    Organization,
    Group,
    Application,
}

#[cfg(feature = "utoipa")]
mod openapi {
    use utoipa::{
        ToSchema,
        openapi::{RefOr, Schema, Type, schema::SchemaType},
    };

    use crate::user::ActorType;

    impl utoipa::PartialSchema for ActorType {
        fn schema() -> RefOr<Schema> {
            Schema::Object(
                utoipa::openapi::schema::ObjectBuilder::new()
                    .schema_type(SchemaType::new(Type::String))
                    .enum_values(Some([
                        "Person",
                        "Service",
                        "Organization",
                        "Group",
                        "Application",
                    ]))
                    .build(),
            )
            .into()
        }
    }

    impl ToSchema for ActorType {}
}
