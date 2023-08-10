use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
// DateTimeBody
use mongodb::bson::DateTime;
// User document
// {
//   "_id": {
//     "$oid": "64aadff4261f5cd72bb34471"
//   },
//   "auth": {
//     "username": "gabrielmelo",
//     "password": null,
//     "access_control_list": {}
//   },
//   "profile": {
//     "name": "Gabriel Melo",
//     "initials": "GM"
//   },
//   "creationDate": {
//     "$date": {
//       "$date": "2023-07-09T16:27:32.023Z"
//     }
//   }
// }


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserWithAuth {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub auth: Auth,
    pub profile: Profile,
    #[serde(rename = "creationDate")]
    pub creation_date: DateTime,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub profile: Profile,
    #[serde(rename = "creationDate")]
    pub creation_date: DateTime,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
    #[serde(rename = "accessControlList")]
    pub access_control_list: AccessControlList,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccessControlList {}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub initials: String,
}