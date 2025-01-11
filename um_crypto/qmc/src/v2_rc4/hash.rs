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
pub fn hash<T: AsRef<[u8]>>(key: T) -> f64 {
    let mut hash = 1u32;
    for &v in key.as_ref().iter() {
        if v == 0 {
            continue;
        }

        let next_hash = hash.wrapping_mul(v as u32);
        if next_hash == 0 || next_hash <= hash {
            break;
        }

        hash = next_hash;
    }

    hash.into()
}

#[test]
fn test_hash() {
    let expected = 4045008896.0;
    let actual = hash(b"hello world");
    assert_eq!(expected, actual);
}
