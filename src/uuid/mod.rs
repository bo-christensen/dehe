use uuid::Uuid;

pub fn generate_new_uuid() -> String {
    let new_uuid = Uuid::new_v4();
    "u".to_string() + new_uuid.to_string().replace("-","_").as_str()
}