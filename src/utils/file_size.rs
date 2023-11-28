
#[derive(Debug,PartialEq)]
enum ByteType{
    Byte,
    Killobyte,
    MegaByte,
    GigaByte,
    TeraByte,
    PetaByte,
    Exabyte,
    ZettaByte,
    YottaByte,
}

impl std::fmt::Display for ByteType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ByteType::Byte => {write!(f,"B")},
            ByteType::Killobyte =>{write!(f,"KB")} ,
            ByteType::MegaByte => {write!(f,"MB")},
            ByteType::GigaByte => {write!(f,"GB")},
            ByteType::TeraByte => {write!(f,"TB")},
            ByteType::PetaByte => {write!(f,"PB")},
            ByteType::Exabyte => {write!(f,"EB")},
            ByteType::ZettaByte => {write!(f,"ZB")},
            ByteType::YottaByte => {write!(f,"YB")},
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct FileSize{
    bytes:u64,
    simple_bytes:u64,
    byte_type:ByteType,
}

impl FileSize{
    pub fn new(bytes:u64) -> FileSize{
        let reduce_factor = FileSize::reduce_factor(bytes);
        let simple_bytes = bytes / (1024u64.pow(reduce_factor as u32));
        
        FileSize { 
            bytes, 
            simple_bytes: simple_bytes, 
            byte_type: FileSize::get_type(reduce_factor), 
        }
    }

    fn reduce_factor(mut bytes:u64) -> u64{
        let mut current_denominator = 0;
        while bytes >= 1024{
            bytes /= 1024;
            current_denominator += 1
        }
        if current_denominator > 8{
            current_denominator = 8
        }
        return current_denominator
    }

    fn get_type(denominator:u64) -> ByteType {
        match denominator{
            0 => ByteType::Byte,
            1 => ByteType::Killobyte,
            2 => ByteType::MegaByte,
            3 => ByteType::GigaByte,
            4 => ByteType::TeraByte,
            5 => ByteType::PetaByte,
            6 => ByteType::Exabyte,
            7 => ByteType::ZettaByte,
            8 => ByteType::YottaByte,
            //unreachable under current configuration. also idk if a 128-bit operating system exists so cant really even have the two above either.
            9..=u64::MAX => ByteType::ZettaByte
        }
    }
}

impl std::fmt::Display for FileSize{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {}",self.simple_bytes, self.byte_type)
    }
}


#[cfg(test)]
mod tests{
    use crate::utils::file_size::{FileSize, ByteType};

    #[test]
    fn test1(){
        assert_eq!(FileSize{bytes:35860585789,simple_bytes:33,byte_type:ByteType::GigaByte},FileSize::new(35860585789));
    }
}