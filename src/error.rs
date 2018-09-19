use std::io;
use toml::de;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to parse the project manifest. {:?}", _0)]
    ManifestParse(#[cause] de::Error),
    #[fail(display = "Failed to build the project. {:?}", _0)]
    BuildFailed(#[cause] io::Error),
}
