//! # File Integrity Library
//!
//! `file_integrity` is a Rust library that provides functions for ensuring the integrity of files.
//!
//! This library allows you to calculate MD5 hashes of file contents, generate JSON reports of file hashes,
//! and more. It is designed to help developers verify the integrity of files in various scenarios.
//!
//! ## Usage
//!
//! To use this library, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! file_integrity = "0.1"
//! ```
//!
//! Then, you can import and use the functions provided by the library in your code.
//!
//! ## Examples
//!
//! ```rust
//! use file_integrity::{list_files, hash_file_list, write_json_file} ;
//! fn main() {
//!     let folder_path = "~/" ;
//!     let nbs_of_file = list_files(&folder_path);
//!     println!("INFOS: Number of files: {nbs_of_file}"); 
//!     let hashs = hash_file_list();
//!     let name = "output.json" ; 
//!     write_json_file(&hashs, &name);
//! }
//! ```
//!
//! For more detailed information and examples, refer to the documentation of individual functions and modules.
//!
//! # License
//!
//! This project is licensed under the terms of the MIT license.
//!
//! For further details, please refer to the [LICENSE](https://github.com/Akmot9/file_integrity/blob/main/LICENSE) file.
//!
//! # Contributing
//!
//! Contributions to this library are welcome! Please follow the guidelines outlined in the [CONTRIBUTING](https://github.com/Akmot9/file_integrity) file.
mod data;
mod hash;
mod json;
mod file_listing;

pub use hash::{hash_file, hash_file_list, calculate_md5_hash};
pub use json::write_json_file;
pub use data::{FileList, FileInfo};
pub use file_listing::list_files ;