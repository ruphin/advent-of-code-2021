use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};

fn main() {
    let numbers = read("../input.txt").unwrap();
    let mut count = 0;
    for i in 0..(numbers.len() - 3) {
        if numbers[i] < numbers[i + 3] {
            count += 1;
        }
    }
    println!("{}", count)
}

// function takes path in the form of string slice and returns enum
// which contains vector of integers on success or IO error type, see `std::io::Error`
fn read(path: &str) -> Result<Vec<i64>, io::Error> {
    let file = File::open(path)?; // open file by given path
                                  // wrap file into generic buffered reader, it will use 4 KB buffer internally
                                  // to reduce number of syscalls, thus improving performance
    let br = BufReader::new(file);
    // create an empty vector, type of the stored elements will be inferred
    let mut v = Vec::new();
    // br.lines() creates an iterator over lines in the reader
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
    for line in br.lines() {
        // IO operations generally can return error, we check if got
        // an error,in which case we return this error as the function result
        let line = line?;
        let n = line
            .trim() // trim "whitespaces"
            .parse() // call `str::parse::<i64>(&self)` method on the trimmed line, which parses integer
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error (e.g. for string "abc"), here if we got it, we convert it to `std::io::Error` type and return it as function result
        v.push(n); // push acquired integer to the vector
    }
    Ok(v) // everything is Ok, return vector
}
