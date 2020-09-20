extern crate protoc_rust;

use pbdemo::protos::Person::Person;
use protobuf;
use protobuf::Message;

fn main() {
    let mut person1 = Person::new();
    person1.set_name("lfq".to_string());
    person1.set_age(12);
    person1.set_gender("male".to_string());
    let person_bytes = person1.write_to_bytes().unwrap();

    let person2 = protobuf::parse_from_bytes::<Person>(&person_bytes).unwrap();
    println!("Person.name: {:?}", person2.get_name());
    println!("Person.age: {:?}", person2.get_age());
    println!("Person.gender: {:?}", person2.get_gender());
}
