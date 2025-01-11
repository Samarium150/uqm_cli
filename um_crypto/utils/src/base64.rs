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
use base64::engine::{DecodePaddingMode, GeneralPurpose as Base64Engine, GeneralPurposeConfig};
use base64::{alphabet, Engine};

pub use base64::DecodeError;

/// Don't add padding when encoding and require no padding when decoding.
pub const ENGINE: Base64Engine = Base64Engine::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent),
);

/// Don't add padding when encoding and require no padding when decoding.
pub const ENGINE_URL_SAFE: Base64Engine = Base64Engine::new(
    &alphabet::URL_SAFE,
    GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent),
);

pub fn encode<T>(data: T) -> String
where
    T: AsRef<[u8]>,
{
    ENGINE.encode(data)
}

pub fn decode<T>(data: T) -> Result<Vec<u8>, DecodeError>
where
    T: AsRef<[u8]>,
{
    ENGINE.decode(data)
}

pub fn decode_overwrite<T>(data: &mut T) -> Result<&[u8], DecodeError>
where
    T: AsMut<[u8]> + ?Sized,
{
    let data = data.as_mut();
    let decoded = decode(&mut data[..])?;
    let len = decoded.len();
    data[..len].copy_from_slice(&decoded);
    Ok(&data[..len])
}

pub fn encode_url_safe<T>(data: T) -> String
where
    T: AsRef<[u8]>,
{
    ENGINE_URL_SAFE.encode(data)
}

pub fn decode_url_safe<T>(data: T) -> Result<Vec<u8>, DecodeError>
where
    T: AsRef<[u8]>,
{
    ENGINE_URL_SAFE.decode(data)
}