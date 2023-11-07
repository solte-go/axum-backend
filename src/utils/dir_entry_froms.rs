use crate::prelude::*;
use std::fs::DirEntry;
use crate::error::{Error, Result};

impl TryFrom<W<&DirEntry>> for String {
    type Error = Error;
    fn try_from(val: W<&DirEntry>) -> Result<String> {
        val.0
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| {
                Error::Generic(format!("Error: {:?}", val.0
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_dir;
    use serial_test::serial;

    use crate::error::{Error, Result};

    #[serial]
    #[test]
    fn test_dir_try_from() -> Result<()>{
        for entry in read_dir("./")?
            .filter_map(|e| e.ok()) {
            println!("{entry:?}");
        }
        Ok(())
    }

}