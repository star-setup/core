pub mod crud;
pub use crud::{
  add_profile, has_profile, insert_profile, list_profiles, remove_profile, remove_profile_entry,
};
pub mod display;
pub use display::print_profile_details;
