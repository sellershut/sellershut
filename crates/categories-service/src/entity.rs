use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    // Main category fields
    pub subcategory_ids: Option<Vec<String>>,
    pub subcategory_ap_ids: Option<Vec<String>>,
    pub subcategory_names: Option<Vec<String>>,
    pub subcategory_images: Option<Vec<String>>, // Optional image_url, could be NULL
    pub subcategory_locals: Option<Vec<bool>>,
    pub parent_id: Option<String>, // Optional parent_id, could be NULL
    pub subcategory_createds: Option<Vec<OffsetDateTime>>,
    pub subcategory_updateds: Option<Vec<OffsetDateTime>>,
}
