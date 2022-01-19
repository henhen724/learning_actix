use mongodb::bson::doc;
// use pbkdf2::{
//     password_hash,
//     password_hash::{
//         HasherError, Ident, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
//     },
//     Params, Pbkdf2,
// };
// use rand_core::OsRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    #[serde(skip)]
    hash: String,
    #[serde(skip)]
    salt: String,
}

// impl User {
//     pub fn set_password(&mut self, entered_password: String) -> Result<(), HasherError> {
//         let salt_string = SaltString::generate(&mut OsRng);
//         self.salt = format!("{:x?}", salt_string.as_bytes());
//         let hash_bytes = Pbkdf2.hash_password(
//             entered_password.as_bytes(),
//             Some(Ident::new("")),
//             None,
//             Params {
//                 rounds: 1000,
//                 output_length: 64,
//             },
//             salt_string.as_salt(),
//         )?;
//         self.hash = format!("{:x?}", hash_bytes);
//         Ok(())
//     }
//     pub fn check_password(&self, entered_password: String) -> bool {
//         let rslt = PasswordHash::new(&self.hash).ok();
//         match rslt {
//             Some(parsed_hash) => Pbkdf2
//                 .verify_password(entered_password.as_bytes(), &parsed_hash)
//                 .is_ok(),
//             None => {
//                 println!("Failed parse password hash {:?}", self.hash);
//                 false
//             }
//         }
//     }
// }
