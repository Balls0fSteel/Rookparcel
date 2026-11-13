//! Stored secrets: PBKDF2-HMAC-SHA256 password hashes (RFC 8018), HMAC-SHA256 (RFC 2104) and a
//! constant-time comparison. The Web UI's password and API key and the desktop windows' lock
//! PIN (D69) are kept this way; moved here from nox's `webui::auth` so the GUIs that do not
//! link nox can use them.

use super::Sha256;
use crate::util::random::crypto_random_bytes;

/// PBKDF2 iterations for new hashes (OWASP's 2023 figure for PBKDF2-HMAC-SHA256 is 600,000;
/// this is what a small NAS answers in well under a second).
pub const DEFAULT_ITERATIONS: u32 = 100_000;

const BLOCK: usize = 64;

/// HMAC-SHA256 (RFC 2104).
pub fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    let mut k = [0u8; BLOCK];
    if key.len() > BLOCK {
        k[..32].copy_from_slice(&Sha256::digest(key).0);
    } else {
        k[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; BLOCK];
    let mut opad = [0x5cu8; BLOCK];
    for i in 0..BLOCK {
        ipad[i] ^= k[i];
        opad[i] ^= k[i];
    }
    let mut inner = Sha256::default();
    inner.update(&ipad).update(msg);
    let inner = inner.finalize();
    let mut outer = Sha256::default();
    outer.update(&opad).update(&inner.0);
    outer.finalize().0
}

/// PBKDF2 with HMAC-SHA256 (RFC 8018 section 5.2).
pub fn pbkdf2_sha256(password: &[u8], salt: &[u8], iterations: u32, out: &mut [u8]) {
    for (block, chunk) in out.chunks_mut(32).enumerate() {
        let mut msg = salt.to_vec();
        msg.extend_from_slice(&(block as u32 + 1).to_be_bytes());
        let mut u = hmac_sha256(password, &msg);
        let mut t = u;
        for _ in 1..iterations.max(1) {
            u = hmac_sha256(password, &u);
            for (a, b) in t.iter_mut().zip(u) {
                *a ^= b;
            }
        }
        chunk.copy_from_slice(&t[..chunk.len()]);
    }
}

/// Compares in time that depends only on the lengths.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn unhex(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok()).collect()
}

/// `n` random bytes from the OS RNG, as hex.
pub fn random_hex(n: usize) -> String {
    let mut b = vec![0u8; n];
    crypto_random_bytes(&mut b);
    hex(&b)
}

/// A stored password: `pbkdf2-sha256:<iterations>:<salt hex>:<hash hex>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PasswordHash {
    iterations: u32,
    salt: Vec<u8>,
    hash: Vec<u8>,
}

impl PasswordHash {
    pub fn new(password: &str, iterations: u32) -> PasswordHash {
        let mut salt = vec![0u8; 16];
        crypto_random_bytes(&mut salt);
        let mut hash = vec![0u8; 32];
        pbkdf2_sha256(password.as_bytes(), &salt, iterations, &mut hash);
        PasswordHash { iterations, salt, hash }
    }

    pub fn verify(&self, password: &str) -> bool {
        let mut h = vec![0u8; self.hash.len()];
        pbkdf2_sha256(password.as_bytes(), &self.salt, self.iterations, &mut h);
        constant_time_eq(&h, &self.hash)
    }

    pub fn parse(s: &str) -> Option<PasswordHash> {
        let mut it = s.split(':');
        if it.next()? != "pbkdf2-sha256" {
            return None;
        }
        let iterations = it.next()?.parse().ok().filter(|&n| n > 0)?;
        let salt = unhex(it.next()?)?;
        let hash = unhex(it.next()?)?;
        (it.next().is_none() && !hash.is_empty() && hash.len() <= 64).then_some(PasswordHash {
            iterations,
            salt,
            hash,
        })
    }
}

impl std::fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pbkdf2-sha256:{}:{}:{}", self.iterations, hex(&self.salt), hex(&self.hash))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unhex_str(s: &str) -> Vec<u8> {
        unhex(&s.replace(' ', "")).unwrap()
    }

    #[test]
    fn hmac_rfc4231() {
        // test case 2
        assert_eq!(
            hmac_sha256(b"Jefe", b"what do ya want for nothing?").to_vec(),
            unhex_str("5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843")
        );
        // test case 6: a key longer than the block
        assert_eq!(
            hmac_sha256(&[0xaa; 131], b"Test Using Larger Than Block-Size Key - Hash Key First").to_vec(),
            unhex_str("60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54")
        );
    }

    #[test]
    fn pbkdf2_rfc7914() {
        let mut out = [0u8; 64];
        pbkdf2_sha256(b"passwd", b"salt", 1, &mut out);
        assert_eq!(
            out.to_vec(),
            unhex_str(
                "55ac046e56e3089fec1691c22544b605f94185216dde0465e68b9d57c20dacbc\
                 49ca9cccf179b645991664b39d77ef317c71b845b1e30bd509112041d3a19783"
            )
        );
    }

    #[test]
    fn password_hashes() {
        let h = PasswordHash::new("secret", 10);
        assert!(h.verify("secret"));
        assert!(!h.verify("Secret"));
        let s = h.to_string();
        assert!(s.starts_with("pbkdf2-sha256:10:"));
        assert!(!s.contains("secret"));
        assert_eq!(PasswordHash::parse(&s), Some(h));
        for bad in ["", "pbkdf2-sha256:0:00:00", "md5:1:00:00", "pbkdf2-sha256:1:0:00", "pbkdf2-sha256:1:00:"]
        {
            assert_eq!(PasswordHash::parse(bad), None, "{bad}");
        }
        assert_ne!(PasswordHash::new("secret", 10), PasswordHash::new("secret", 10)); // salted
    }

    #[test]
    fn constant_time_eq_compares_whole_slices() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"ab"));
        assert!(constant_time_eq(b"", b""));
        assert_eq!(random_hex(6).len(), 12);
    }
}
