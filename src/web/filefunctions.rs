use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub fn split_file_into_chunks(
    filename: &str,
    chunk_size: u64,
) -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open(format!("./uploads/{}", filename))?;
    let file_size = file.metadata()?.len();

    if file_size <= chunk_size {
        let file_chunk_names = vec![filename.to_string()];

        return Ok(file_chunk_names);
    }

    let num_chunks = (file_size + chunk_size - 1) / chunk_size;

    let mut chunk_filenames = Vec::new();
    for chunk_num in 0..num_chunks {
        let start_pos = chunk_num * chunk_size;
        let end_pos = std::cmp::min(start_pos + chunk_size, file_size);

        // Open a new file for the chunk with automatic closing
        let mut chunk_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("./uploads/chunks/{}_{}", filename, chunk_num))?;

        // Seek to the desired position in the original file
        file.seek(SeekFrom::Start(start_pos))?;

        // Use BufReader for buffered reading
        let mut reader = std::io::BufReader::new(file.try_clone()?);
        let mut bytes_read = 0;
        let mut buffer = [0u8; 4096];
        loop {
            let read_size = reader.read(&mut buffer)?;
            if read_size == 0 {
                break;
            }
            chunk_file.write_all(&buffer[..read_size])?;
            bytes_read += read_size;
            if bytes_read >= end_pos as usize - start_pos as usize {
                break;
            }
        }

        // Add the chunk filename to the vector
        chunk_filenames.push(format!("{}_{}", filename, chunk_num));

        println!("Created chunk: {} ({} bytes)", filename, bytes_read);
    }

    Ok(chunk_filenames)
}
pub fn reassemble_file_from_chunks(filename: &str) -> Result<(), std::io::Error> {
    let mut assembled_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(filename)?;

    let mut chunk_num = 0;
    loop {
        let chunk_filename = format!("{}_{}", filename, chunk_num);

        // Check if the chunk file exists
        if !std::path::Path::new(&chunk_filename).exists() {
            break;
        }

        let mut chunk_file = File::open(chunk_filename.clone())?;

        // Read data from the chunk file and write it to the assembled file
        let mut buffer = [0u8; 4096]; // Adjust buffer size as needed
        loop {
            let read_size = chunk_file.read(&mut buffer)?;
            if read_size == 0 {
                break;
            }
            assembled_file.write_all(&buffer[..read_size])?;
        }

        // Delete the processed chunk file
        std::fs::remove_file(chunk_filename.clone())?;

        chunk_num += 1;
        println!("Processed and deleted chunk: {}", chunk_filename);
    }

    println!("File reassembled successfully!");

    Ok(())
}
