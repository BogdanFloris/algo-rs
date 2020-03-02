/// Utility used for reading from stdin.
use std::io;
use std::str;
use std::fs::File;

/// Reads white space separated tokens one at a time.
pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl <R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Scanner {
            reader,
            buffer: Vec::new(),
        }
    }

    /// Use "turbofish" syntax token::<T>() to select data type of next token.
    ///
    /// # Panics
    ///
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().unwrap();
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).unwrap();
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

pub fn scanner_from_file(filename: &str) -> Scanner<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    Scanner::new(io::BufReader::new(file))
}

pub fn writer_to_file(filename: &str) -> io::BufWriter<File> {
    let file = File::create(filename).unwrap();
    io::BufWriter::new(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;
    use std::io::Write as IOWrite;

    #[test]
    fn test_memory_scanner() {
        let input = "50 8".as_bytes();
        let mut scanner = Scanner::new(input);
        let mut out = String::new();

        let x = scanner.token::<i32>();
        let y = scanner.token::<i32>();
        writeln!(out, "Test: {}", x - y).ok();

        assert_eq!(out, "Test: 42\n");
    }

    #[test]
    fn test_io_scanner() {
        let (stdin, stdout) = (io::stdin(), io::stdout());
        let mut scanner = Scanner::new(stdin.lock());
        let mut out = io::BufWriter::new(stdout.lock());

        // If true, blocks test runs since you need to write to stdin
        if false {
            let x = scanner.token::<i32>();
            let y = scanner.token::<i32>();
            writeln!(out, "Test: {}", x - y).ok();
            out.flush().unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn test_panic_file() {
        let mut scanner = scanner_from_file("input.txt");
        let mut out = writer_to_file("out.txt");

        let x = scanner.token::<i32>();
        let y = scanner.token::<i32>();
        writeln!(out, "Test {}", x - y).ok();
    }
}
