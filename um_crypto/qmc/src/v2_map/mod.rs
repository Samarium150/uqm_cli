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
mod key;

use crate::v1::cipher::{qmc1_transform, V1_KEY_SIZE};
use crate::v2_map::key::key_compress;
use anyhow::Result;

#[derive(Debug, PartialEq, Clone)]
pub struct QMC2Map {
    key: [u8; V1_KEY_SIZE],
}

impl QMC2Map {
    pub fn new<T: AsRef<[u8]>>(key: T) -> Result<Self> {
        let key = key_compress(key)?;
        Ok(Self { key })
    }

    pub fn decrypt<T>(&self, data: &mut T, offset: usize)
    where
        T: AsMut<[u8]> + ?Sized,
    {
        for (i, datum) in data.as_mut().iter_mut().enumerate() {
            *datum = qmc1_transform(&self.key, *datum, offset + i);
        }
    }
}

#[test]
fn test_decrypt() {
    let key = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .chain(b'0'..=b'9')
        .cycle()
        .take(325)
        .collect::<Vec<u8>>();

    let cipher = QMC2Map::new(key).expect("should not fail");
    let mut actual = [
        0x00u8, 0x9e, 0x41, 0xc1, 0x71, 0x36, 0x00, 0x80, 0xf4, 0x00, 0x75, 0x9e, 0x36, 0x00, 0x14,
        0x8a,
    ];
    cipher.decrypt(&mut actual, 32760);
    assert_eq!(actual, [0u8; 0x10]);
}
