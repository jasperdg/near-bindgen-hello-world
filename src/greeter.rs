use std::string::String;
use near_bindgen::{near_bindgen, ENV, MockedEnvironment};
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

#[test]
     fn set_get_greeting() {
     ENV.set(Box::new(MockedEnvironment::new()));
     let account_id = b"alice";
     ENV.as_mock().set_originator_id(account_id.to_vec());
     let mut contract = Greeter::default();
     contract.set_greeting("G'day".to_owned());
     assert_eq!("G'day, world!", contract.greet());
}