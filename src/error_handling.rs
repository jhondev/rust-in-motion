use serde::Deserialize;
use serde_json;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::result;

//**************Handling Result */
#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}

pub fn handle_result() {
    let first = serde_json::from_str::<Person>(
        r#"{
        "name": "Margaret Hamilton",
    }"#,
    );

    let first_inner = match first {
        Ok(inner) => inner,
        Err(_) => Person {
            name: "unknown".to_owned(), // it will enter cause the comma in the json
        },
    };

    println!("first's name = {:?}", first_inner.name);
}

//***********Generalizing error */
// use box to propagate different error types
fn num_threads() -> result::Result<usize, Box<dyn Error>> {
    let s = env::var("NUM_THREADS")?; // error propagation with ?
    let n: usize = s.parse()?; // multiple error propagation with ?
    Ok(n + 1)
}

pub fn box_error() {
    match num_threads() {
        Ok(num) => println!("the number of threads is {}", num),
        Err(e) => println!("error getting num threads: {}", e),
    }
}

//**********Custom Error */
#[derive(Debug)]
pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),
}

pub type Result<T> = result::Result<T, DocumentServiceError>;

impl Error for DocumentServiceError {}

impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => write!(f, "Limit exceeded"),
            Io(ref io) => write!(f, "I/O error: {}", io),
        }
    }
}

impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

pub fn create_document(filename: &str) -> Result<File> {
    let last_min_docs = 5;
    let max_docs = 3;
    if last_min_docs > max_docs {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}
