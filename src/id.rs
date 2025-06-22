extern crate id_derive;
use serde::Serialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub use id_derive::HasId;

/// Trait for generating and accessing a unique id for a struct based on its fields.
/// This trait is intended to be used with #[derive(HasId)].
pub trait HasId {
    fn id(&self) -> &str;
    fn generate_id(&self) -> String;
}

/// Helper function to hash a struct's type name and serialized fields to create a unique id.
pub fn generate_struct_id<T: Serialize>(instance: &T) -> String {
    let type_name = std::any::type_name::<T>();
    let serialized = serde_json::to_string(instance).unwrap_or_default();
    let uuid = Uuid::new_v4();
    let mut hasher = Sha256::new();
    hasher.update(type_name.as_bytes());
    hasher.update(serialized.as_bytes());
    hasher.update(uuid.as_bytes());
    let hash = hasher.finalize();
    format!("{:x}", hash)
} 