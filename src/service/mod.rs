// Include the user_service submodule.
mod user_service;

// Re-export the UserService struct for easy access from outside this module.
//pub use user_service::UserService;
pub use self::user_service::UserService;

// You can also declare any shared traits or utilities used across different services here.
// For example, if you had more services (e.g., AuthenticationService), you could define common traits here.

//
