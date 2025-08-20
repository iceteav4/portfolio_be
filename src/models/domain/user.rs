use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, EnumString, Display)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Pending,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub status: UserStatus,
    pub email: String,
    pub phone_number: Option<String>,
    pub name: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

// impl User {
//     pub fn from_row(row: Option<UserRow>) -> Option<Self> {
//         row.map(|row| Self {
//             id: row.id,
//             status: row.status.parse().unwrap(),
//             email: row.email,
//             phone_number: row.phone_number,
//             name: row.name,
//             created_at: row.created_at,
//             updated_at: row.updated_at,
//         })
//     }
// }
