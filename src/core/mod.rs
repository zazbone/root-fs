use std::fs::File;
use std::io;
use std::iter::zip;

#[derive(Debug)]
enum RUsize {
    Ru32,
    Ru64
}

impl Default for RUsize {	
	fn default() -> Self {
		RUsize::Ru64
	}
}


#[derive(Default, Debug)]
pub struct Header {
    _tagexist: bool,
    version: u32,
    ptr_size: RUsize,
    begin: u64,
    end: u64,
    seek_free: u64,
    nbytes_free: u64,
    nfree: u64,
}

impl Header {
	pub fn from_source(source: Source) -> io::Result<Self> {
		match source {
			Source::Local(mut f) => Self::from_file(&mut f)
		}
	}

	fn from_file(file: &mut File) -> io::Result<Self> {
		let mut buffer = [0u8; 4];

		// try to read tag
		let mut tagexist = true;
		io::Read::read(file, &mut buffer)?;
		for (c, wc) in zip(&buffer[0..4], b"root") {
			if c != wc {
				tagexist = false;
				break;
			}
		}

		Ok(
			Header {
				_tagexist: tagexist,
				version: 0,
				ptr_size: RUsize::Ru64,
				begin: 0,
				end: 0,
				seek_free: 0,
				nbytes_free: 0,
				nfree: 0,
			}
		)
	}

	pub fn had_root_tag(&self) -> bool {
		self._tagexist
	}
}

pub enum Source {
	Local(File)
}