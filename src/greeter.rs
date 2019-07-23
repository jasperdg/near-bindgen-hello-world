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
   pub fn set_greeter(&mut self, greeting: String){
        self.greeting = greeting;
   }
   pub fn greet(&self) -> String {
        let what_to_greet: &str = "world";     
        return format!("{}, {}!", self.greeting, what_to_greet);
   }    
}