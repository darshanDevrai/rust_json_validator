
use rust_json_validator::validator::Validator;
use std::fs;


fn main() {
 
    let raw_schema = fs::read_to_string("schema.json").unwrap();
    let  raw_json =  fs::read_to_string("myjson.json").unwrap();

    // let raw_schema = fs::read_to_string("school_schema.json").unwrap();
    // let  raw_json =  fs::read_to_string("school_data.json").unwrap();

    // let raw_schema = fs::read_to_string("studi_schema.json").unwrap();
    // let raw_json =  fs::read_to_string("studi_data.json").unwrap();

    let error_string = Validator::validate_json(raw_schema, raw_json);

    // println!("Errors are {:?}", json);
    fs::write("errors.json", error_string).unwrap();

}
