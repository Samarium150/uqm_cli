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
use crate::footer::{musicex_v1, FooterParseError, Metadata, MetadataParser};
use byteorder::{ByteOrder, LE};

#[derive(Debug, Clone, PartialEq)]
pub struct PcV2MusicEx {
    /// Resource identifier (`.mid`)
    pub mid: String,

    /// The actual file name used for `ekey` lookup (`.file.media_mid` + extension).
    pub media_filename: String,
}

impl MetadataParser for PcV2MusicEx {
    fn from_byte_slice(payload: &[u8]) -> Result<Option<Metadata>, FooterParseError> {
        if payload.len() < 16 {
            Err(FooterParseError::BufferTooSmall(16))?;
        }

        if let Some(payload) = payload.strip_suffix(b"musicex\x00") {
            let (payload, version) = payload.split_at(payload.len() - 4);
            let version = LE::read_u32(version);

            return match version {
                1 => musicex_v1::parse_v1(payload),
                _ => Err(FooterParseError::PCv2InvalidVersion(version))?,
            };
        }

        Ok(None)
    }
}
