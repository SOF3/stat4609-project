use std::cell::RefCell;
use std::collections::{self, BTreeMap};
use std::fs;
use std::io::{self, BufRead, Write};

use chrono::Datelike;
use chrono::NaiveDate as Date;

type Set = collections::HashSet<UserMovie>;

fn main() -> io::Result<()> {
    let mut output = io::BufWriter::new(fs::File::create("output.npy")?);

    let data = read_all_data()?;

    output.write_all(b"\x93NUMPY\x01\x00")?;

    let mut header = get_header(data.len());
    let length = 10 + header.len();
    let padding = 16 - (length % 16);
    if padding != 16 {
        header.extend(std::iter::repeat(b' ').take(padding));
    }

    output.write_all(&(header.len() as u16).to_le_bytes())?;
    output.write_all(&header)?;

    for (idx, row) in data.iter().enumerate() {
        write_cell(&mut output, row.user)?;
        write_cell(&mut output, row.movie)?;
        write_cell(&mut output, row.rating)?;
        write_cell(&mut output, row.year)?;
        write_cell(&mut output, row.doy)?;
        write_cell(&mut output, row.dow)?;
        write_cell(&mut output, row.probed)?;

        if (idx + 1) % 1000000 == 0 {
            println!("Wrote {} lines", idx + 1);
        }
    }

    Ok(())
}

fn write_cell(f: &mut impl Write, v: impl Into<u32>) -> io::Result<()> {
    f.write_all(&v.into().to_le_bytes())
}

#[derive(Default)]
struct Index {
    index: RefCell<IndexRaw>,
}
impl Index {
    fn resolve(&self, value: u32) -> u32 {
        let mut borrow = self.index.borrow_mut();
        borrow.resolve_mut(value)
    }
}

#[derive(Default)]
struct IndexRaw {
    cache: BTreeMap<u32, u32>,
    counter: u32,
}
impl IndexRaw {
    fn resolve_mut(&mut self, value: u32) -> u32 {
        let Self { cache, counter } = self;
        *cache.entry(value).or_insert_with(|| {
            let value = *counter;
            *counter += 1;
            value
        })
    }
}

#[derive(Debug)]
#[repr(C)]
struct Row {
    user: u32,
    movie: u32,
    rating: u32,
    year: u32,
    doy: u32,
    dow: u32,
    probed: u32,
}

fn read_all_data() -> io::Result<Vec<Row>> {
    let mut data = Vec::new();
    let user_index = Index::default();
    let movie_index = Index::default();

    let probe = Probe {
        file: io::BufReader::new(fs::File::open("probe.txt")?),
        current_movie: 0,
        buf: Vec::new(),
        // user_index: &user_index,
        movie_index: &movie_index,
        lines: 0,
    };
    let probe: Set = probe.collect();

    read_data(
        io::BufReader::new(fs::File::open("combined_data_1.txt")?),
        &mut data,
        &user_index,
        &movie_index,
        &probe,
    )?;
    read_data(
        io::BufReader::new(fs::File::open("combined_data_2.txt")?),
        &mut data,
        &user_index,
        &movie_index,
        &probe,
    )?;
    read_data(
        io::BufReader::new(fs::File::open("combined_data_3.txt")?),
        &mut data,
        &user_index,
        &movie_index,
        &probe,
    )?;
    read_data(
        io::BufReader::new(fs::File::open("combined_data_4.txt")?),
        &mut data,
        &user_index,
        &movie_index,
        &probe,
    )?;
    Ok(data)
}

fn read_data(
    mut file: impl BufRead,
    data: &mut Vec<Row>,
    user_index: &Index,
    movie_index: &Index,
    probe: &Set,
) -> io::Result<()> {
    let mut buf = Vec::with_capacity(64);

    let mut current_movie = 0;

    while let Ok(size) = file.read_until(b'\n', {
        buf.clear();
        &mut buf
    }) {
        let mut buf = &buf[..size];
        buf = buf.strip_suffix(b"\n").unwrap_or(buf);

        let line = std::str::from_utf8(buf).expect("Non-ascii line");

        if let Some(movie_id_str) = line.strip_suffix(":") {
            let movie_id_raw: u32 = movie_id_str.parse().expect("Not a movie ID");
            current_movie = movie_index.resolve(movie_id_raw);
        } else if line.contains(',') {
            let mut cells = line.split(',');
            let user_id_str = cells.next().unwrap();
            let user_id_raw: u32 = user_id_str.parse().expect("Not a user ID");
            let user_id = user_index.resolve(user_id_raw);

            let rating_str = cells.next().unwrap();
            let rating: u32 = rating_str.parse().expect("Not a rating");

            let date = cells.next().unwrap();
            let mut tokens = date.split('-');
            let year: u32 = tokens.next().unwrap().parse().unwrap();
            let month = tokens.next().unwrap().parse().unwrap();
            let day = tokens.next().unwrap().parse().unwrap();
            let chrono = Date::from_ymd(year as i32, month, day);
            let doy = chrono.ordinal0();
            let dow = chrono.weekday().num_days_from_monday();

            let probed = probe.contains(&UserMovie {
                user_raw: user_id_raw,
                movie: current_movie,
            });
            let probed = if probed { 1 } else { 0 };

            let row = Row {
                user: user_id,
                movie: current_movie,
                rating,
                year,
                doy,
                dow,
                probed,
            };
            data.push(row);

            if data.len() % 1000000 == 0 {
                println!("Read {} lines", data.len());
            }
        } else {
            println!("Empty line!");
            break;
        }
    }

    Ok(())
}

struct Probe<'t, R: io::BufRead> {
    file: R,
    current_movie: u32,
    buf: Vec<u8>,
    // user_index: &'t Index,
    movie_index: &'t Index,
    lines: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct UserMovie {
    user_raw: u32,
    movie: u32,
}

impl<'t, R: io::BufRead> Iterator for Probe<'t, R> {
    type Item = UserMovie;

    fn next(&mut self) -> Option<UserMovie> {
        while let Ok(size) = self.file.read_until(b'\n', {
            self.buf.clear();
            &mut self.buf
        }) {
            if size == 0 {
                break;
            }

            let mut buf = &self.buf[..size];
            buf = buf.strip_suffix(b"\n").unwrap_or(buf);

            let line = std::str::from_utf8(buf).expect("Non-ascii line");

            if let Some(movie_id_str) = line.strip_suffix(":") {
                let movie_id_raw: u32 = movie_id_str.parse().expect("Not a movie ID");
                self.current_movie = self.movie_index.resolve(movie_id_raw);
            } else if !line.is_empty() {
                let user_id_raw: u32 = line.parse().expect("Not a user ID");
                // let user_id = self.user_index.resolve(user_id_raw);

                self.lines += 1;
                if self.lines % 1000000 == 0 {
                    println!("Read {} lines of probe", self.lines);
                }


                let ret = UserMovie {
                    user_raw: user_id_raw,
                    movie: self.current_movie,
                };
                return Some(ret);
            }
        }

        None
    }
}

fn get_header(num_rows: usize) -> Vec<u8> {
    let mut header = Vec::new();
    write!(
        &mut header,
        r#"{{'descr': '<i4', 'fortran_order': False, 'shape': ({num_rows}, 7), }}"#,
        num_rows = num_rows
    )
    .expect("Failure writing Vec<u8>");

    header
}
