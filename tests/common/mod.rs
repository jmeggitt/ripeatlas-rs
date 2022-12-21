use format_serde_error::SerdeError;
use rayon::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use std::io;
use std::io::BufRead;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

pub mod bzip2;

pub fn debug_read<T, R: BufRead>(reader: &mut R) -> io::Result<()>
where
    for<'a> T: Deserialize<'a>,
{
    let mut count = 0;

    let mut buffer = String::new();
    while reader.read_line(&mut buffer)? != 0 {
        count += 1;
        if let Err(e) = serde_json::from_str::<T>(&buffer) {
            let err = SerdeError::new(buffer.to_owned(), e);

            let raw_json = serde_json::from_str::<Value>(&buffer)?;
            let prettified = serde_json::to_string_pretty(&raw_json)?;

            eprintln!("{}", buffer);
            eprintln!("{}", prettified);

            eprintln!("{}", err);
            panic!("Received parsing error on line {}", count);
        }

        buffer.clear();
    }

    eprintln!("Finished after successfully parsing {} items", count);
    Ok(())
}

pub fn debug_read_rayon<T, R: BufRead + Send>(reader: &mut R) -> io::Result<()>
where
    for<'a> T: Deserialize<'a>,
{
    let count = AtomicU64::new(0);

    reader
        .lines()
        .par_bridge()
        .try_for_each(|buffer| -> io::Result<()> {
            let buffer = buffer?;
            count.fetch_add(1, SeqCst);

            if let Err(e) = serde_json::from_str::<T>(&buffer) {
                let err = SerdeError::new(buffer.to_owned(), e);

                let raw_json = serde_json::from_str::<Value>(&buffer)?;
                let prettified = serde_json::to_string_pretty(&raw_json)?;

                eprintln!("{}", buffer);
                eprintln!("{}", prettified);

                eprintln!("{}", &err);

                return Err(io::Error::new(io::ErrorKind::Other, err));
            }

            Ok(())
        })?;

    eprintln!(
        "Finished after successfully parsing {} items",
        count.load(SeqCst)
    );
    Ok(())
}
