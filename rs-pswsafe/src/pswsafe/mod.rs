use lsx::BufSha256;
use crate::{FileNotFound, PswSafeError};
use crate::PswSafeError::{FileNotSupported, IterationsNotInitialized, NumberOfIterationsNotFound, SaltNotFound};

// EOF: The ASCII characters "PWS3-EOFPWS3-EOF" (note that this is
// exactly one block long), unencrypted. This is an implementation convenience
// to inform the application that the following bytes are to be processed
// differently.
const EOF:&[u8] = b"PWS3-EOFPWS3-EOF";

const PSW3_IDENTIFIER: &[u8] = b"PWS3";

const SALT_SIZE: usize = 32;
const KEY_SIZE: usize = 32;
const ITER_SIZE: usize = 4;

#[derive(Debug)]
pub struct PswSafe {
    salt: [u8; SALT_SIZE],
    // ITER is the number of iterations on the hash function to calculate stretch_key
    iter: u32,
    stretch_key_sha256: [u8; KEY_SIZE],
    /* B1 and B2 are two 128-bit blocks encrypted with Twofish [TWOFISH]
    using P' as the key, in ECB mode. These blocks contain the 256 bit
    random key K that is used to encrypt the actual records. (This has the
    property that there is no known or guessable information on the
    plaintext encrypted with the passphrase-derived key that allows an
    attacker to mount an attack that bypasses the key stretching
    algorithm.) */
    b1: [u8; 16],
    b2: [u8; 16],
    /* B3 and B4 are two 128-bit blocks encrypted with Twofish using P' as the
    key, in ECB mode. These blocks contain the 256 bit random key L that is
    used to calculate the HMAC (keyed-hash message authentication code) of the
    encrypted data. See description of EOF field below for more details.
    Implementation Note: K and L must NOT be related. */
    b3: [u8; 16],
    b4: [u8; 16],
    // IV is the 128-bit random Initial Value for CBC mode.
    iv:  [u8; 16],
    // HDR: The database header. The header consists of one or more typed
    // fields (as defined in section 3.2), beginning with the Version type
    // field, and terminated by the 'END' type field. The version number
    // and END fields are mandatory. Aside from these two fields, no order is
    // assumed on the field types.
    raw_hdr: Vec<u8>,
    // HMAC: The 256-bit keyed-hash MAC, as described in RFC2104, with SHA-
    // 256 as the underlying hash function. The value is calculated over all of
    // the plaintext fields, that is, over all the data stored in all fields
    // (starting from the version number in the header, ending with the last field
    // of the last record). The key L, as stored in B3 and B4, is used as the hash
    // key value.
    hmac: [u8; 32]
}

impl PswSafe {
    pub fn new() -> PswSafe {
        PswSafe {
            salt: [0; 32],
            iter: 0,
            stretch_key_sha256: [0; 32],
            b1: [0; 16],
            b2: [0; 16],
            b3: [0; 16],
            b4: [0; 16],
            iv: [0; 16],
            raw_hdr: vec![],
            hmac: [0; 32]
        }
    }

    // TAG is the sequence of 4 ASCII characters "PWS3". This is to serve as a
    // quick way for the application to identify the database as a PasswordSafe
    // version 3 file. This tag has no cryptographic value.
    fn check_tag(&self, bytes: &[u8]) -> Result<(), PswSafeError> {
        if bytes[..PSW3_IDENTIFIER.len()].eq(PSW3_IDENTIFIER) {
            return Ok(());
        }
        Err(FileNotSupported)
    }

    pub fn check(&self, bytes: &[u8]) -> Result<usize, PswSafeError> {
        self.check_tag(bytes)?;
        let position_eof = match bytes.windows(EOF.len()).position(| w | w == EOF) {
            Some(p) => p,
            None => return Err(FileNotFound)
        };
        Ok(position_eof)
    }

    fn set_salt(&mut self, bytes: &[u8]) -> Result<usize, PswSafeError>{
        if bytes.len() < PSW3_IDENTIFIER.len() + self.salt.len() {
            return Err(SaltNotFound)
        }
        self.salt.copy_from_slice(&bytes[PSW3_IDENTIFIER.len()..(SALT_SIZE + PSW3_IDENTIFIER.len())]);
        Ok(self.salt.len())
    }

    fn set_iter(&mut self, bytes: &[u8]) -> Result<u32, PswSafeError> {
        let start = SALT_SIZE + PSW3_IDENTIFIER.len();
        let end = SALT_SIZE + PSW3_IDENTIFIER.len() + ITER_SIZE;
        let mut buff: [u8; 4] = [0; 4];
        if bytes.len() < end {
            return Err(NumberOfIterationsNotFound)
        }
        buff.copy_from_slice(&bytes[start..end]);
        self.iter = u32::from_le_bytes(buff);
        return Ok(self.iter.clone())
    }

    fn stretch_key(&mut self, mut pw: Vec<u8>) -> Result<[u8; 32], PswSafeError> {
        if self.iter == 0 {
            return Err(IterationsNotInitialized)
        }
        let mut hasher = BufSha256::new();
        hasher.update(pw.as_slice());
        pw.fill(0);
        let mut r = hasher.finish(&self.salt);
        for n in 0..self.iter {
            r = hasher.finish(&r);
            println!("{} => {:?}", n, r);
        }
        self.stretch_key_sha256 = r;
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn strecht_key_match() {
        let mut data_buf = Vec::new();
        File::open("DevTest.psafe3").expect("Failed to open Test File").read_to_end(&mut data_buf);

        let mut safe = PswSafe::new();
        safe.set_salt(&data_buf);
        safe.set_iter(&data_buf);
        let pw_vec = "PswSafe123".as_bytes().to_vec();
        let key = safe.stretch_key(pw_vec);
        assert!(key.is_ok());
        let key_data = key.unwrap();
        let start = SALT_SIZE + PSW3_IDENTIFIER.len() + ITER_SIZE;
        let end = SALT_SIZE + PSW3_IDENTIFIER.len() + ITER_SIZE + KEY_SIZE;
        let key_org = &data_buf[start..end];
        assert!(key_org.to_vec().eq(key_data.as_slice()));
    }

    #[test]
    fn check_reports_valid_file() {
        let mut data_buf = Vec::new();
        File::open("DevTest.psafe3").expect("Failed to open Test File").read_to_end(&mut data_buf);

        let safe = PswSafe::new();
        assert!(safe.check(&data_buf).is_ok())
    }

    #[test]
    fn check_tag_finds_tag() {
        let safe = PswSafe::new();
        let tag_s = safe.check_tag(PSW3_IDENTIFIER);
        assert!(tag_s.is_ok())
    }

    #[test]
    fn check_tag_finds_tag_in_file() {
        let mut data_buf = Vec::new();
        File::open("DevTest.psafe3").expect("Failed to open Test File").read_to_end(&mut data_buf);

        let safe = PswSafe::new();
        let tag_s = safe.check_tag(data_buf.as_slice());
        if tag_s.is_err() {
            println!("{:?} is not {:?}", data_buf.as_slice()[..PSW3_IDENTIFIER.len()].to_vec(), PSW3_IDENTIFIER.to_vec());
        }
        assert!(tag_s.is_ok())
    }

    #[test]
    fn set_salt_from_file() {
        let mut data_buf = Vec::new();
        File::open("DevTest.psafe3").expect("Failed to open Test File").read_to_end(&mut data_buf);

        let mut safe = PswSafe::new();
        let tag_s = safe.set_salt(data_buf.as_slice());

        if tag_s.is_err() {
            println!("salt to small");
        }
        assert_eq!(safe.salt.eq(&[0;32]), false);

        assert!(tag_s.is_ok());
    }

    #[test]
    fn set_iter_from_file() {
        let mut data_buf = Vec::new();
        File::open("DevTest.psafe3").expect("Failed to open Test File").read_to_end(&mut data_buf);

        let mut safe = PswSafe::new();
        let tag_s = safe.set_iter(data_buf.as_slice());

        if tag_s.is_err() {
            println!("salt to small");
        }
        assert_eq!(safe.iter == 0, false);

        assert!(tag_s.is_ok());
    }
}