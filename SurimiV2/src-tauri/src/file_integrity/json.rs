use std::fs::File;
use std::io::Write;
use serde_json;
use crate::file_integrity::data::FileList;
use my_logger::log;

/// Write a `FileList` struct to a JSON file.
///
/// This function serializes a given `FileList` struct into a JSON-formatted string,
/// and writes it to the specified output file. The JSON is formatted with indentation
/// for readability.
///
/// If the serialization fails or if there are issues with file I/O, appropriate error
/// messages are logged.
///
/// # Arguments
///
/// * `hash_set` - The `FileList` struct to be serialized and written to the JSON file.
///
/// # Examples
///
/// ```
/// use file_integrity::{write_json_file, FileList};
///
/// let file_list = FileList {
///     date: "2023-08-10".to_string(),
///     files: vec![/* FileInfo entries */],
/// };
/// let file_name = "output.json" ;
/// write_json_file(&file_list, &file_name);
/// ```
///
/// # Errors
///
/// This function can return an error if serialization to JSON fails or if there are issues
/// with file I/O operations.

pub fn write_json_file(hash_set: &FileList, output_file_name: &str) {
    log!("STATUS: Writing json: Please wait...");
    let json_output = match serde_json::to_string_pretty(&hash_set) {
        Ok(output) => output,
        Err(err) => {
            log!("ERROR: Failed to serialize to JSON: {}", err);
            return;
        }
    };

    let output_file = output_file_name;
    match File::create(output_file) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(json_output.as_bytes()) {
                log!("ERROR: Failed to write JSON to file: {}", err);
            } else {
                log!("STATUS: Writing json: Success !");
            }
        }
        Err(err) => {
            log!("ERROR: Failed to create output file: {}", err);
        }
    }
}