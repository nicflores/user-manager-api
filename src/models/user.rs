use serde::{Deserialize, Serialize};

/// Represents a user in the system.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub bucket_name: String,
    pub public_ssh_key: String,
    pub aws_role_arn: String,
}

impl User {
    // You can add methods here related to the User struct,
    // such as validation methods or helper functions to manipulate user data.
}
