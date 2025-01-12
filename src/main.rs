/*!
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
use anyhow::{bail, Result};
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::exit;
use umc_qmc::{footer, QMCv2Cipher};

const BUFFER_SIZE: usize = 4 * 1024 * 1024;

mod utils;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Verbose printing
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// The output directory
    #[arg(short, long, value_name = "DIR")]
    output: Option<PathBuf>,

    /// The database containing the QMCv2 encryption keys
    #[arg(short = 'D', long = "db")]
    db: PathBuf,

    /// Replace original files
    #[arg(short, long, default_value_t = false)]
    replace: bool,

    /// The input directory
    input: PathBuf,
}

impl Cli {
    // noinspection SpellCheckingInspection
    pub fn run(&self) -> Result<i32> {
        if !fs::metadata(&self.input)?.is_dir() {
            bail!("{:?} is not a directory", &self.input);
        }
        let tasks: Vec<PathBuf> = fs::read_dir(&self.input)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .map(|path| fs::canonicalize(path).unwrap())
            .collect();
        let database = utils::load_db(&self.db)?;
        let output: PathBuf = match &self.output {
            Some(path) => match fs::metadata(&path) {
                Ok(metadata) => {
                    if !metadata.is_dir() {
                        bail!("{:?} is not a directory", &path);
                    }
                    path.clone()
                }
                Err(_) => {
                    fs::create_dir_all(&path)?;
                    path.clone()
                }
            },
            None => self.input.clone(),
        };
        for task in tasks {
            if self.verbose {
                println!("processing {:?}", task);
            }
            let mut file = File::open(&task)?;
            let filename = utils::get_filename(&task)?;
            if !filename.contains(".mflac2") {
                continue;
            }
            let mut buffer = vec![0u8; footer::INITIAL_DETECTION_LEN];
            file.seek(SeekFrom::End(-(footer::INITIAL_DETECTION_LEN as i64)))?;
            file.read_exact(&mut buffer)?;
            let size = file.stream_position()?;
            file.seek(SeekFrom::Start(0))?;
            let (footer_size, ekey) = match footer::from_byte_slice(&buffer) {
                Ok(Some(metadata)) => {
                    if self.verbose {
                        println!("{}: {:?}", filename, metadata);
                    }
                    (metadata.size, metadata.ekey)
                }
                Ok(None) => {
                    eprintln!("could not find any qmc metadata.");
                    (0usize, None)
                }
                Err(err) => {
                    eprintln!("failed to parse qmc metadata: {}", err);
                    (0usize, None)
                }
            };
            let key: Vec<u8> = match ekey {
                None => match database.get(&filename) {
                    None => {
                        eprintln!("could not find ekey for {}", filename);
                        continue;
                    }
                    Some(ekey) => umc_qmc::ekey::decrypt(ekey)?,
                },
                Some(ekey) => umc_qmc::ekey::decrypt(ekey)?,
            };
            let cipher = QMCv2Cipher::new(key)?;
            let mut output = output.clone();
            output.push(filename.clone().replace(".mflac2", ""));
            let mut output = BufWriter::new(File::create(output)?);
            let mut buffer = vec![0u8; BUFFER_SIZE];
            let reader = BufReader::with_capacity(BUFFER_SIZE, file);
            let mut reader = reader.take(size - footer_size as u64);
            let mut offset = 0usize;
            while let Ok(n) = reader.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                cipher.decrypt(&mut buffer[..n], offset);
                output.write_all(&buffer[..n])?;
                offset += n;
            }
            if self.replace {
                fs::remove_file(task)?;
            }
            if self.verbose {
                println!("{} decrypted", filename)
            }
        }
        Ok(0)
    }
}

fn main() {
    let cli = Cli::parse();
    let code = cli.run().unwrap_or_else(|err| {
        eprintln!("run command failed: {}", err);
        -1
    });
    exit(code);
}
