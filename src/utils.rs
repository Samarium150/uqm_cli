/*
 * Copyright (c) 2025 Samarium150
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
use anyhow::Result;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn get_filename(path: &PathBuf) -> Result<String> {
    Ok(path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
        .expect("Invalid filename"))
}

// noinspection SpellCheckingInspection
pub fn load_db(path: &PathBuf) -> Result<HashMap<String, String>> {
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let mut stmt = conn.prepare("SELECT file_path, ekey FROM audio_file_ekey_table")?;
    let map = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<rusqlite::Result<HashMap<_, _>>>()?
        .into_iter()
        .map(|(key, value)| (get_filename(&PathBuf::from(key)).unwrap(), value))
        .collect();
    Ok(map)
}
