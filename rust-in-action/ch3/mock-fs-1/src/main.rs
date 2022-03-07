//! Simulating files one step at a time
use std::fmt;
use std::fmt::{Display};

#[derive(Debug,PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED")
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, ({})>",
        self.name, self.state)
    }
}

/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl File {
    /// New files are assumed to be empty
    /// but a name is required
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: vec![],
            state: FileState::Closed,
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    pub fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;

    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;

    Ok(f)
}

fn main() {
    let mut f5 = File::new("5.txt");

    let mut buffer: Vec<u8> = vec![];
    if f5.read(&mut buffer).is_err() {
        println!("Error checking working");
    }

    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f5);
    println!("{} is {} bytes long", &f5.name, f5_length);
    println!("{}", text);
}
