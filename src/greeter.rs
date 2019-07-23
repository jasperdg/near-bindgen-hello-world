use std::string::String;
use near_bindgen::{near_bindgen};
use serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(Default, Serialize, Deserialize)]
pub struct Greeter {
    greeting: String
}

#[near_bindgen]
impl Greeter {
   pub fn set_greeting(&mut self, greeting: String){
        self.greeting = greeting;
   }
   pub fn greet(&self) -> String {
        let mut full_greeting: String = self.greeting.to_owned();
        full_greeting.push_str(", world!");
        return full_greeting;
   }
}