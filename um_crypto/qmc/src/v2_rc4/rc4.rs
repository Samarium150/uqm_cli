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
use std::ops::Rem;

#[derive(Debug, Clone)]
pub struct RC4 {
    state: Box<[u8]>,
    i: usize,
    j: usize,
}

fn init_state(key: &[u8]) -> Box<[u8]> {
    let n = key.len();
    let mut state: Box<[u8]> = (0..n).map(|i| i as u8).collect();

    let mut j = 0usize;
    for i in 0..state.len() {
        j = (j + usize::from(state[i]) + usize::from(key[i % n])) % n;
        state.swap(i, j);
    }

    state
}

impl RC4 {
    pub fn new<K: AsRef<[u8]>>(key: K) -> Self {
        Self {
            state: init_state(key.as_ref()),
            i: 0,
            j: 0,
        }
    }

    pub fn generate(&mut self) -> u8 {
        let n = self.state.len();
        self.i = self.i.wrapping_add(1).rem(n);
        self.j = self.j.wrapping_add(self.state[self.i].into()).rem(n);
        self.state.swap(self.i, self.j);

        let i = usize::from(self.state[self.i]);
        let j = usize::from(self.state[self.j]);
        let index = (i + j) % n;

        self.state[index]
    }

    pub fn derive<T>(&mut self, buffer: &mut T)
    where
        T: AsMut<[u8]> + ?Sized,
    {
        for item in buffer.as_mut().iter_mut() {
            *item ^= self.generate();
        }
    }
}

#[test]
fn test_rc4() {
    let mut rc4 = RC4::new(b"this is a test key");
    let rc4_copy = rc4.clone();

    let mut data = *b"hello world";
    rc4.derive(&mut data[..]);

    assert_ne!(rc4.state, rc4_copy.state);
    assert_eq!(&data, b"\x68\x75\x6b\x64\x64\x24\x7f\x60\x7c\x7d\x60")
}
