pub mod crud;
pub use crud::{add_profile, has_profile, insert_profile, remove_profile, remove_profile_entry};
pub mod display;
pub use display::{list_profiles, print_profile_details};
