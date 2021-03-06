use std::time::SystemTime;
use super::schema::{images, products};
use serde_derive::Deserialize;

#[derive(QueryableByName, Debug)]
#[table_name="images"]
pub struct Images {
    pub img_id:             String,
    pub pro_type:           Option<String>,
    pub img_name:           Option<Vec<String>>,
    pub created_at:         SystemTime,    
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name="images"]
pub struct Image {
    pub img_id:    String,
    pub pro_type:  String,
    pub img_name:  Vec<String>
}

#[derive(QueryableByName, Debug)]
#[table_name="products"]
pub struct Products {
    pub pro_id:             Option<String>,
    pub pro_name:           Option<String>,
    pub pro_type:           Option<String>,
    pub pro_description:    Option<String>,
    pub pro_quantity:       Option<i32>,
    pub created_at:         SystemTime,
}

#[derive(Insertable, Deserialize)]
#[table_name="products"]
pub struct Product {
    pub pro_id:             String,
    pub pro_name:           String,
    pub pro_type:           String,
    pub pro_description:    String,
    pub pro_quantity:       i32
}


// pub struct Pro_Img {
//     pro_id:             String,
//     pro_name:           String,
//     pro_type:           String,
//     pro_description:    String,
//     pro_quantity:
// }