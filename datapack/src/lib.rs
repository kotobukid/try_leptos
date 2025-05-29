use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub name: String,
    pub id: u32,
}

pub fn export_features() -> HashMap<String, Vec<Card>> {
    let mut feature_map: HashMap<String, Vec<Card>> = HashMap::new();

    feature_map.insert(
        "feature1".to_string(),
        vec![Card {
            name: "card1".to_string(),
            id: 1,
        }],
    );
    feature_map.insert(
        "feature2".to_string(),
        vec![Card {
            name: "card2".to_string(),
            id: 2,
        }],
    );
    feature_map.insert(
        "feature3".to_string(),
        vec![Card {
            name: "card3".to_string(),
            id: 3,
        }],
    );
    feature_map.insert(
        "feature4".to_string(),
        vec![Card {
            name: "card4".to_string(),
            id: 4,
        }],
    );
    feature_map.insert(
        "feature5".to_string(),
        vec![Card {
            name: "card5".to_string(),
            id: 5,
        }],
    );
    feature_map.insert(
        "feature6".to_string(),
        vec![Card {
            name: "card6".to_string(),
            id: 6,
        }],
    );

    feature_map
}
