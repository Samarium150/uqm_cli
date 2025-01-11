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
use md5::{Digest, Md5};

pub fn md5<T: AsRef<[u8]>>(buffer: T) -> [u8; 16] {
    Md5::digest(buffer).into()
}

pub fn md5_2<T1: AsRef<[u8]>, T2: AsRef<[u8]>>(buffer1: T1, buffer2: T2) -> [u8; 16] {
    let mut md5_digest = Md5::default();
    md5_digest.update(buffer1);
    md5_digest.update(buffer2);
    md5_digest.finalize().into()
}
