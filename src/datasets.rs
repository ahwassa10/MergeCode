use std::{env::current_dir, fs::File, io::{self, BufRead, BufReader, BufWriter, Result, Write}};

pub fn write_sequence_file(filename: &str, start: u64, end: u64) -> Result<()> {
    let mut path = current_dir()?;
    path.push("datasets");
    path.push(filename);

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    for i in start..end {
        writeln!(&mut writer, "{i}")?;
    }
    writer.flush()?;
    
    Ok(())
}

pub fn read_sequence_file(filename: &str) -> Result<Vec<u64>>{
    let mut path = current_dir()?;
    path.push("datasets");
    path.push(filename);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut output = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let val: u64 = line.trim().parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Cannot parse {:?} as a u64: {}", line, e)
            )
        })?;
        output.push(val);
    }

    Ok(output)
}


#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use super::*;

    #[test]
    fn test_gen_dataset() {
        let test = "test_file";
        write_sequence_file(&test, 0, 10).unwrap();
        let read_back = read_sequence_file(&test).unwrap();

        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(read_back, expected);

        let mut delete_path = current_dir().unwrap();
        delete_path.push("datasets");
        delete_path.push(test);

        remove_file(delete_path).unwrap();
    }
}