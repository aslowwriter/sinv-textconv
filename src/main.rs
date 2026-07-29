mod cli;
mod error;

use clap::Parser;
use flate2::bufread::ZlibDecoder;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

use crate::cli::CliArgs;
use crate::error::SinvTextconvError;

fn main() -> Result<(), SinvTextconvError> {
    let res = inner_main();
    // not the most elegant of solutions, but we just want to exit
    // cleanly if the pipe is closed prematurely since that's perfectly acceptable behavior so
    // we have to do this little wrapper
    // if anything else goes wrong we'll just exit in whatever way
    // is appropriate
    if let Err(SinvTextconvError::IoError(ref e)) = res {
        if e.kind() == ErrorKind::BrokenPipe {
            Ok(())
        } else {
            res
        }
    } else {
        res
    }
}

pub fn inner_main() -> Result<(), SinvTextconvError> {
    let args: CliArgs = CliArgs::parse();

    if !args.file.is_file() {
        return Err(SinvTextconvError::ArgNotAFile(format!(
            "{}",
            args.file.display()
        )));
    }

    let file = File::open(args.file)?;
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    let mut buffered_header_reader = BufReader::new(file);

    let mut sphinx_version_line = String::new();
    buffered_header_reader.read_line(&mut sphinx_version_line)?;
    write!(stdout_handle, "{sphinx_version_line}")?;

    let mut project_line = String::new();
    buffered_header_reader.read_line(&mut project_line)?;
    write!(stdout_handle, "{project_line}")?;

    let mut version_line = String::new();
    buffered_header_reader.read_line(&mut version_line)?;
    write!(stdout_handle, "{version_line}")?;

    let mut compression_line = String::new();
    buffered_header_reader.read_line(&mut compression_line)?;
    write!(stdout_handle, "{compression_line}")?;

    let zlib_reader = BufReader::new(ZlibDecoder::new(buffered_header_reader));

    for line in zlib_reader.lines() {
        writeln!(stdout_handle, "{}", line?)?;
    }

    Ok(())
}
