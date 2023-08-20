use walkdir::WalkDir;
use std::fs::File;
use std::io::{Write, BufWriter};
use my_logger::{log, logw};

/// Recursively list files in a directory and write their paths to a text file.
///
/// This function traverses the specified directory and its subdirectories,
/// listing all the file paths found. It then writes these file paths to a
/// text file named "file_list.txt" in the same directory where the function
/// is called from.
///
/// The function also counts the number of files found and returns the count.
/// Any encountered errors during the listing process are logged.
///
/// # Arguments
///
/// * `folder_to_list` - The path to the directory to be recursively listed.
///
/// # Returns
///
/// The number of files listed.
///
/// # Examples
///
/// ```
/// use file_integrity::list_files;
///
/// let folder_path = "/path/to/your/directory";
/// let nbs_of_file = list_files(folder_path);
/// println!("Number of Files: {}", nbs_of_file);
/// ```
///
/// # Errors
///
/// This function logs any errors that occur during the listing and writing process.


pub fn list_files(folder_to_list: &str) -> i32 {
    logw!("STATUS: Listing files: Please wait...");
    let mut count = 0;
    let mut filenames = Vec::new();

    for file in WalkDir::new(&folder_to_list).into_iter().filter_map(|file| file.ok()) {
        let path = file.path().display().to_string();
    
        if !path.starts_with("/sys/") &&
           !path.starts_with("/dev/") &&
           !path.starts_with("/run/") &&
           !path.starts_with("/proc/") {
            
            if let Ok(metadata) = file.metadata() {
                if metadata.is_file() {
                    count += 1;
                    filenames.push(path);
                }
            } else if let Some(error) = file.metadata().err() {
                // Log the error message using log macro
                log!("ERROR: Can't access file: {:?}: {}", path, error);
            }
        }
    }    // Write filenames to file.txt
    
    if let Ok(file) = File::create("file_list.txt") {
        let mut writer = BufWriter::new(file);
        for filename in &filenames {
            if let Err(error) = writeln!(writer, "{}", filename) {
                log!("ERROR:  writing to file_list.txt: {}", error);
            }
        }
    } else {
        log!("ERROR: Failed to create file_list.txt");
    }
    println!("\n") ;
    logw!("STATUS: List of files: Succes !");
    count
}