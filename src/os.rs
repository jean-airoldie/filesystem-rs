use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};
use std::path::{Path, PathBuf};

#[cfg(feature = "temp")]
use tempdir;

use FileSystem;
#[cfg(feature = "temp")]
use TempDir;

#[cfg(feature = "temp")]
#[derive(Debug)]
pub struct OsTempDir(tempdir::TempDir);

#[cfg(feature = "temp")]
impl TempDir for OsTempDir {
    fn path(&self) -> &Path {
        self.0.path()
    }
}

#[derive(Clone, Debug, Default)]
pub struct OsFileSystem {}

impl OsFileSystem {
    pub fn new() -> Self {
        OsFileSystem {}
    }
}

impl FileSystem for OsFileSystem {
    #[cfg(feature = "temp")]
    type TempDir = OsTempDir;

    fn current_dir(&self) -> Result<PathBuf> {
        env::current_dir()
    }

    fn set_current_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        env::set_current_dir(path)
    }

    fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_dir()
    }

    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_file()
    }

    fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir(path)
    }

    fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir_all(path)
    }

    fn remove_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::remove_dir(path)
    }

    fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::remove_dir_all(path)
    }

    fn write_file<P, B>(&self, path: P, buf: B) -> Result<()>
        where P: AsRef<Path>,
              B: AsRef<[u8]>
    {
        let mut file = File::create(path)?;
        file.write_all(buf.as_ref())
    }

    fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        let mut contents = Vec::<u8>::new();
        let mut file = File::open(path)?;

        file.read_to_end(&mut contents)?;

        Ok(contents)
    }

    fn create_file<P, B>(&self, path: P, buf: B) -> Result<()>
        where P: AsRef<Path>,
              B: AsRef<[u8]>
    {
        let mut file = OpenOptions::new().write(true)
            .create_new(true)
            .open(path)?;

        file.write_all(buf.as_ref())
    }

    fn readonly<P: AsRef<Path>>(&self, path: P) -> Result<bool> {
        let metadata = fs::metadata(path)?;
        let permissions = metadata.permissions();

        Ok(permissions.readonly())
    }

    fn set_readonly<P: AsRef<Path>>(&self, path: P, readonly: bool) -> Result<()> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();

        permissions.set_readonly(readonly);
        file.set_permissions(permissions)
    }

    #[cfg(feature = "temp")]
    fn temp_dir<S: AsRef<str>>(&self, prefix: S) -> Result<Self::TempDir> {
        tempdir::TempDir::new(prefix.as_ref()).map(OsTempDir)
    }
}