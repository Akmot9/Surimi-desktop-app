use std::fs::File;
use std::io::{Read, BufReader, BufRead, self, Write};
use md5::{Md5, Digest};
use crate::file_integrity::data::{FileInfo, FileList};
use my_logger::{log, logw, logd};
use chrono::Local;

/// Calculate hashes for a list of files from a text file.
///
/// This function reads a list of file paths from the specified text file, calculates
/// MD5 hashes for the contents of each file, and returns a `FileList` struct containing
/// the date of the operation and a vector of `FileInfo` entries.
///
/// Only files not starting with '/sys/', '/dev/', '/run/', or '/proc/' are included in the
/// hash calculation process.
///
/// # Arguments
///
/// * `filename` - The path to the text file containing the list of file paths to hash.
///
/// # Returns
///
/// A `FileList` struct containing the date and a vector of `FileInfo` entries.
///
/// # Examples
///
/// ```
/// use file_integrity::{hash_file_list, FileList};
///
/// let file_list = hash_file_list(); // hashs file_list.txt by defalt if no ags given
/// println!("Date: {}", file_list.date);
/// println!("Number of Files: {}", file_list.files.len());
/// ```
///
/// # Errors
///
/// This function can return an error if the file cannot be opened or read, or if there are
/// issues reading lines from the file.

pub fn hash_file_list(total: & i32) -> FileList {
    // Hide the cursor.
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();
    logw!("STATUS: Hashing files: Please wait...");
    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut file_hashed = 0;
    let mut liste = FileList {
        date,
        files: vec![],
    };
    
    // Use the provided filename or use the default value "file_list.txt"
    let actual_filename = "file_list.txt";
    
    let file = match File::open(actual_filename) {
        Ok(file) => file,
        Err(error) => {
            logd!("ERROR: Can't open file '{}': {}", actual_filename, error);
            // Increment the counter for successfully hashed files
            file_hashed += 1;
            print!("\r{file_hashed} / {total}");
            io::stdout().flush().unwrap();
            return liste;
        }
    };
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(error) => {
                log!("ERROR: Can't read line from file '{}': {}", actual_filename, error);
                // Increment the counter for successfully hashed files
                file_hashed += 1;
                let percentage = (file_hashed * 100) / total;
                print!("\r{file_hashed} / {total}: {percentage}%");
                io::stdout().flush().unwrap();
                continue;
            }
        };

        // Exclude files that start with '/sys/', '/dev/', '/run/', and '/proc/' from being hashed
        if !line.starts_with("/sys/") && 
            !line.starts_with("/dev/")
            && !line.starts_with("/run/") 
            && !line.starts_with("/proc/") {
                let file_info = hash_file(line);
                liste.files.push(file_info);

                // Increment the counter for successfully hashed files
                file_hashed += 1;
                let percentage = (file_hashed * 100) / total;
                print!("\r{}% - {}/{}", percentage, file_hashed, total);
                io::stdout().flush().unwrap();
        }
    }
    log!("STATUS: Hashing files: Success !");
    log!("infos: Hashed files: {file_hashed} !");
    // Show the cursor again.
    print!("\x1B[?25h");
    io::stdout().flush().unwrap();
    liste
}

/// Calculate the hash of a file's contents.
///
/// This function reads the contents of the specified file, calculates its MD5 hash,
/// and returns the result as a `FileInfo` struct.
///
/// If the file cannot be opened, read, or its contents cannot be decoded as UTF-8,
/// an error `FileInfo` with "none" as the hash is returned.
///
/// # Arguments
///
/// * `filename` - The path to the file for which to calculate the MD5 hash.
///
/// # Returns
///
/// A `FileInfo` struct containing the filename and MD5 hash of the file's contents.
///
/// # Examples
///
/// ```
/// use file_integrity::{hash_file, FileInfo};
///
/// let filename = "path/to/your/file.txt".to_string();
/// let file_info = hash_file(filename);
/// println!("Filename: {}", file_info.filename);
/// println!("MD5 Hash: {}", file_info.md5_hash);
/// ```
///
/// # Errors
///
/// This function can return an error if the file cannot be opened, read, or its contents
/// cannot be decoded as UTF-8.

pub fn hash_file(filename: String) -> FileInfo {
    let mut file = match File::open(filename.clone()) {
        Ok(file) => file,
        Err(_) => {
            logw!("ERROR: Can't open file: {}", filename);
            return FileInfo {
                filename,
                md5_hash: String::from("none"),
            };
        }
    };
    let mut buffer = Vec::new();
    if let Err(error) = file.read_to_end(&mut buffer) {
        log!("ERROR: Can't read file: {}", error);
        return FileInfo {
            filename,
            md5_hash: String::from("none"),
        };
    }
    let input = match String::from_utf8(buffer) {
        Ok(input) => input,
        Err(_) => {
            //log!("ERROR: Can't decode file contents: {}", error);
            return FileInfo {
                filename,
                md5_hash: String::from("none"),
            };
        }
    };
    FileInfo {
        filename,
        md5_hash: calculate_md5_hash(&input),
    }
}

/// Calculates the hash of a given input string.
///
/// This function takes an input string and calculates its MD5 hash using the MD5 algorithm.
/// The resulting hash is returned as a hexadecimal string.
///
/// # Arguments
///
/// * `input` - The input string for which to calculate the MD5 hash.
///
/// # Returns
///
/// The MD5 hash as a hexadecimal string.
///
/// # Examples
///
/// ```
/// use file_integrity::calculate_md5_hash;
///
/// let input = "Hello, world!";
/// let hash = calculate_md5_hash(&input.to_string());
/// println!("MD5 hash: {}", hash);
/// ```

pub fn calculate_md5_hash(input: &String) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    let hash_string = format!("{:x}", result);
    hash_string
}
#[doc(hidden)]
#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};
    use tempfile::tempdir;
    use crate::file_integrity::{hash_file, FileInfo};

    use super::calculate_md5_hash;
    #[test]
    fn test_calculate_md5_hash() {
        let input = String::from("hello, world");
        let expected_hash = "e4d7f1b4ed2e42d15898f4b27b019da4";
        let calculated_hash = calculate_md5_hash(&input);
        assert_eq!(calculated_hash, expected_hash);
    }
    
    #[test]
    fn test_hash_file_valid_file() {
    // Create a temporary file with known content
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_file_path = temp_dir.path().join("test_file.txt");
    let mut temp_file = File::create(&temp_file_path).expect("Failed to create temporary file");
    temp_file
        .write_all(b"This is a test file.\n")  // Add a newline character
        .expect("Failed to write to temporary file");

    // Calculate the expected hash value for the content
    let expected_hash = "2d282102fa671256327d4767ec23bc6b";

    // Calculate the hash and compare with the expected value
    let result = hash_file(temp_file_path.display().to_string());
    assert_eq!(result.md5_hash, expected_hash);
}

    #[test]
    fn test_hash_file_nonexistent_file() {
        // Test the hash_file function with a non-existent file path
        // Ensure the "none" hash is generated

        // Define a non-existent file path
        let file_path = "nonexistent_file.txt"; // Adjust the path accordingly

        // Call the function you want to test
        let result = hash_file(file_path.to_string());

        // Check that the result matches the expected FileInfo structure
        let expected_result = FileInfo {
            filename: file_path.to_string(),
            md5_hash: "none".to_string(),
        };
        assert_eq!(result.md5_hash, expected_result.md5_hash);
    }

    #[test]
    fn test_hash_file_list_with_test_files() {
        // Create a temporary test file and test the hash_file_list function
        // Ensure correct hash information is generated for each file
    }

    #[test]
    fn test_hash_file_list_exclude_files() {
        // Test scenarios where certain files should be excluded based on their paths
    }

    // Additional tests for other functions can be added here
}