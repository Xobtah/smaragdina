use std::{fs, io, path::Path};

#[derive(bincode::Encode, bincode::Decode)]
pub struct Sekreto {
    pub dosiero: String,
    pub enhavo: Vec<u8>,
}

impl Sekreto {
    const fn bincode_config() -> bincode::config::Configuration {
        bincode::config::standard()
            .with_little_endian()
            .with_variable_int_encoding()
    }
}

impl TryFrom<&Path> for Sekreto {
    type Error = io::Error;

    fn try_from(valoro: &Path) -> Result<Self, Self::Error> {
        Ok(Self {
            dosiero: valoro.file_name().unwrap().to_str().unwrap().to_string(),
            enhavo: fs::read(valoro)?,
        })
    }
}

impl TryFrom<&Sekreto> for Vec<u8> {
    type Error = bincode::error::EncodeError;

    fn try_from(valoro: &Sekreto) -> Result<Self, Self::Error> {
        bincode::encode_to_vec(valoro, Sekreto::bincode_config())
    }
}

impl TryFrom<Vec<u8>> for Sekreto {
    type Error = bincode::error::DecodeError;

    fn try_from(valoro: Vec<u8>) -> Result<Self, Self::Error> {
        let (tio, _) = bincode::decode_from_slice(&valoro, Sekreto::bincode_config())?;
        Ok(tio)
    }
}
