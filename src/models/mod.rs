// Re-export the User struct so it's directly accessible from the models module.
pub use user::User;

// Include the user submodule.
mod user;

// If you have more models, declare them here in a similar fashion.
// This keeps your models module organized and makes it easy to extend as your application grows.
