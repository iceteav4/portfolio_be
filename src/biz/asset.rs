use crate::models::common::asset::AssetType;

// pub struct AssetBiz {
//     pool: PgPool,
// }

// impl AssetBiz {
//     pub fn new(pool: PgPool) -> Self {
//         Self { pool }
//     }

//     pub async fn create_asset(self, inp: CreateAssetBiz) -> Result<String, AppError> {
//         let asset_repo = AssetRepo::new(self.pool.clone());
//         Ok("Asset created successfully".to_string())
//     }
// }

pub fn generate_asset_id(asset_type: &AssetType, external_id: &str) -> String {
    format!("{}_{}", asset_type.to_string(), external_id.to_lowercase())
}
