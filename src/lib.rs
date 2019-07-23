use std::collections::HashMap;

#[near_bindgen]
#[derive(Default, Serialize, Deserialize)]
pub struct MyContract {
    data: HashMap<u64, u64>
}

// #[near_bindgen]
// impl MyContract {
//    pub fn insert_data(&mut self, key: u64, value: u64) -> Option<u64> {
//        self.data.insert(key)
//    }
//    pub fn get_data(&self, key: u64) -> Option<u64> {
//        self.data.get(&key).cloned()
//    }
// }