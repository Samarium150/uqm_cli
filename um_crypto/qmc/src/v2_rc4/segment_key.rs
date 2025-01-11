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
pub fn get_segment_key(id: u64, seed: u8, hash: f64) -> u64 {
    match seed {
        0 => 0,
        seed => {
            let result = hash / ((id + 1).wrapping_mul(seed.into()) as f64) * 100.0;
            result as u64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_key_nil_seed() {
        assert_eq!(get_segment_key(1, 0, 12345.0), 0);
    }

    #[test]
    fn test_segment_key_123() {
        assert_eq!(get_segment_key(1, 123, 12345.0), 5018);
    }

    #[test]
    fn test_segment_key_large_1() {
        assert_eq!(get_segment_key(51, 35, 516402887.0), 28373784);
    }

    #[test]
    fn test_segment_key_large_2() {
        assert_eq!(get_segment_key(0, 66, 3908240000.0), 5921575757);
    }
}
