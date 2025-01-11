/*!
 * Copyright (c) 2024 Project Unlock Music
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *     http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::v1::cipher::V1_KEY_SIZE;
use crate::QmcCryptoError;

const INDEX_OFFSET: usize = 71214;

pub fn key_compress<T: AsRef<[u8]>>(long_key: T) -> anyhow::Result<[u8; V1_KEY_SIZE]> {
    let long_key = long_key.as_ref();
    if long_key.is_empty() {
        Err(QmcCryptoError::QMCV2MapKeyEmpty)?;
    }

    let n = long_key.len();
    let mut result = [0u8; V1_KEY_SIZE];

    let key_stream = (0..V1_KEY_SIZE).map(|i| {
        let i = (i * i + INDEX_OFFSET) % n;
        let key = long_key[i];
        let shift = ((i + 4) % 8) as u32;
        key.wrapping_shl(shift) | key.wrapping_shr(shift)
    });

    for (key, value) in result.iter_mut().zip(key_stream) {
        *key = value;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let expected = [
            0x79, 0xf4, 0x00, 0x75, 0x9e, 0x36, 0x00, 0x14, 0x8a, 0x63, 0x00, 0xb4, 0xbe, 0x77,
            0x00, 0x17, 0xba, 0x00, 0x37, 0x00, 0x00, 0x00, 0xbf, 0x80, 0x41, 0xbf, 0x83, 0xdd,
            0xbc, 0x5c, 0x02, 0x43, 0x14, 0x82, 0x49, 0x02, 0x00, 0x55, 0xbe, 0x6d, 0xbf, 0x49,
            0x80, 0x8e, 0x43, 0x00, 0xfa, 0x41, 0x67, 0xa8, 0x17, 0xf4, 0xae, 0x16, 0x15, 0x00,
            0xc1, 0x37, 0x82, 0xdd, 0x36, 0x21, 0x38, 0x55, 0x00, 0x79, 0x41, 0x9e, 0x42, 0xc1,
            0x36, 0xfa, 0xcf, 0x35, 0x00, 0x00, 0x41, 0xdd, 0x43, 0x42, 0x17, 0x4d, 0x8e, 0x8a,
            0xdd, 0x00, 0xbe, 0xf5, 0x38, 0xb4, 0xbf, 0x00, 0x7a, 0xcc, 0x4d, 0x02, 0x00, 0xcf,
            0xc1, 0xc1, 0x02, 0xa8, 0x00, 0x16, 0xc1, 0xbf, 0xc2, 0x42, 0x00, 0x49, 0x00, 0xc1,
            0xc2, 0xf5, 0x00, 0x17, 0x41, 0xdc, 0x83, 0xc2, 0x00, 0x9e, 0x41, 0xc1, 0x71, 0x36,
            0x00, 0x80,
        ];
        let key = (b'a'..=b'z')
            .chain(b'A'..=b'Z')
            .chain(b'0'..=b'9')
            .cycle()
            .take(325)
            .collect::<Vec<u8>>();

        let actual = key_compress(&key).expect("should compress ok");
        assert_eq!(expected, actual);
    }
}
