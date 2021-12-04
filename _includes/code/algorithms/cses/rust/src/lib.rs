use std::fmt::{Debug, Display};
use std::io;
use std::str::FromStr;

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    pub fn token<T: FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    pub fn get_spaced_line_vec<T: FromStr>(&mut self) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut items = String::new();

        self.reader.read_line(&mut items).ok().expect("read error");

        items
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect()
    }

    pub fn get_line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input.trim_end().to_string()
    }

    pub fn split_line(&mut self) -> Vec<char> {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input.trim_end().chars().collect()
    }

    pub fn get_matrix(&mut self) -> Vec<Vec<char>> {
        let mut v: Vec<Vec<char>> = vec![];
        let mut input = String::new();
        loop {
            let bytes = self.reader.read_line(&mut input).ok().expect("Read error");
            if bytes == 0 {
                break;
            }
            v.push(input.trim().chars().collect());
            input = String::new();
        }
        v
    }

    pub fn get_line_to_tuple<T: Clone + FromStr>(&mut self) -> (T, T)
    where
        <T as FromStr>::Err: Debug,
        T: Debug,
    {
        let mut items = String::new();

        self.reader.read_line(&mut items).ok().expect("read error");

        let pair: Vec<T> = items
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();

        (pair[0].clone(), pair[1].clone())
    }
}

pub fn print_vec<T: Display>(v: &Vec<T>) {
    for (index, item) in v.iter().enumerate() {
        print!("{}", item);
        if index != v.len() - 1 {
            print!(" ");
        }
    }
}

pub fn print_matrix<T: Display>(m: &Vec<Vec<T>>) {
    for item in m {
        print_vec(&item);
        print!("\n");
    }
}
