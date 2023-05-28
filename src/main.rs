use aes::Aes128;
use base64::{decode, encode};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use clap::{App, Arg};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use ini::Ini;
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn to_plagr(
    file_name: String,
    optional_ext: Option<String>,
    no_backup: bool,
    secure: bool,
) -> std::io::Result<()> {
    let file = std::fs::File::open(&file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(&contents)?;
    let compressed_contents = e.finish()?;

    let mut encoded_string = encode(&compressed_contents);

    let mut rng = rand::thread_rng();
    let key: [u8; 16] = rng.gen();
    let iv: [u8; 16] = rng.gen();
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).expect("Failed to create cipher");

    if secure {
        let encrypted_data = cipher.clone().encrypt_vec(encoded_string.as_bytes());
        encoded_string = base64::encode(&encrypted_data);

        let mut conf = Ini::new();
        conf.with_section(Some("DEFAULT".to_owned()))
            .set("IV_KEY", base64::encode(&iv))
            .set("KEY", base64::encode(&key));
        conf.write_to_file("key.ini").unwrap();
    }

    if !no_backup {
        let backup_file_name = Path::new(&file_name).with_extension("bak");
        std::fs::copy(&file_name, backup_file_name)?;
    }

    let file_ext = {
        Path::new(&file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string()
    };

    let mut plagr_file_name = PathBuf::from(&file_name);
    plagr_file_name.set_extension("plagr");

    let file_ext_top = optional_ext.map_or_else(
        || {
            Path::new(&file_name)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_string()
        },
        |ext| ext.trim_start_matches('.').to_owned(),
    );

    let encoded_extension = if secure {
        let encrypted_extension = cipher.encrypt_vec(file_ext_top.as_bytes());
        base64::encode(&encrypted_extension)
    } else {
        file_ext_top
    };

    let mut plagr_file = std::fs::File::create(&plagr_file_name)?;
    plagr_file.write_all(format!("{}\n{}", encoded_extension, encoded_string).as_bytes())?;

    std::fs::remove_file(&file_name)?;
    std::fs::rename(
        &plagr_file_name,
        file_name.replace(&format!(".{}", file_ext), ".plagr"),
    )?;
    Ok(())
}

fn from_plagr(
    file_name: String,
    keys: Option<HashMap<String, String>>,
    key_path: &str,
    no_cleanup: bool,
    secure: bool,
    new_ext: Option<String>,
) -> std::io::Result<()> {
    let file = File::open(file_name.clone())?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut ext = lines[0].trim().to_string();
    let encoded_string = lines[1..].concat();

    let mut decoded_string = Vec::new();

    let (key, iv) = match keys {
        Some(k) => (
            decode(k.get("KEY").expect("No key provided")).unwrap(),
            decode(k.get("IV_KEY").expect("No IV key provided")).unwrap(),
        ),
        None => {
            let conf = Ini::load_from_file(key_path).unwrap();
            let section = conf.section(Some("DEFAULT".to_owned())).unwrap();
            (
                decode(section.get("KEY").expect("No key provided")).unwrap(),
                decode(section.get("IV_KEY").expect("No IV key provided")).unwrap(),
            )
        }
    };

    let cipher = Aes128Cbc::new_from_slices(&key, &iv).expect("Failed to create cipher");

    if secure {
        let decrypted_extension = cipher
            .clone()
            .decrypt_vec(&decode(&ext).unwrap())
            .expect("Failed to decrypt extension or wrong keys provided");
        ext = String::from_utf8(decrypted_extension).unwrap();

        let decrypted_bytes = cipher
            .decrypt_vec(&decode(&encoded_string).unwrap())
            .expect("Failed to decrypt data or wrong keys provided");

        let decoded_bytes = decode(decrypted_bytes).unwrap_or_else(|_| Vec::new());
        let mut d = GzDecoder::new(&decoded_bytes[..]);
        d.read_to_end(&mut decoded_string)?;
    } else {
        let decoded_bytes = decode(&encoded_string).unwrap_or_else(|_| Vec::new());
        let mut d = GzDecoder::new(&decoded_bytes[..]);
        d.read_to_end(&mut decoded_string)?;
    }

    let ext = new_ext.unwrap_or(ext);

    let output_file = Path::new(&file_name).with_extension("").with_extension(ext);

    let mut file = File::create(output_file.clone())?;
    file.write_all(&decoded_string)?;

    if Path::new(&file_name).exists() {
        fs::remove_file(&file_name)?;
    }

    let backup_file = output_file.with_extension("bak");
    if !no_cleanup && backup_file.exists() {
        fs::remove_file(backup_file)?;
    }

    Ok(())
}

fn main() {
    let app = App::new("plagr")
        .version("1.0")
        .author("Daniel Oppermann <dan0xe@proton.me>")
        .about("Converts files to and from .plagr format.")
        .subcommand(
            App::new("to_plagr")
                .about("Converts a file to .plagr format.")
                .arg(
                    Arg::with_name("file")
                        .help("The file to convert.")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("ext")
                        .long("ext")
                        .value_name("EXT")
                        .help("Optional new file extension for when converting back from .plagr"),
                )
                .arg(
                    Arg::with_name("no-backup")
                        .long("no-backup")
                        .help("Skip creating a backup of the original file."),
                )
                .arg(
                    Arg::with_name("secure")
                        .long("secure")
                        .help("Encrypt the data with a key before compressing and converting."),
                ),
        )
        .subcommand(
            App::new("from_plagr")
                .about("Converts a .plagr file back to its original format.")
                .arg(
                    Arg::with_name("file")
                        .help("The .plagr file to convert back.")
                        .required(true)
                        .index(1),
                )
                .arg(
                   Arg::with_name("keys")
                    .long("keys")
                    .help("Keys for decryption in the format: iv_key=<IV_KEY>,key=<KEY> or a file path specifying the key.ini file.")
                    .takes_value(true),
                )
                .arg(
                Arg::with_name("key-path")
                .long("key-path")
                .help("Path to the key.ini file.")
                .takes_value(true),
            )
                .arg(
                    Arg::with_name("no-cleanup")
                        .long("no-cleanup")
                        .help("Skip deleting the backup file after conversion."),
                )
                .arg(
                    Arg::with_name("secure")
                    .long("secure")
                    .help("Sets the secure flag"),
                )
                .arg(
                    Arg::with_name("ext")
                        .long("ext")
                        .value_name("EXT")
                        .help("Override the file extension when converting back from .plagr"),
                )
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        ("to_plagr", Some(sub_m)) => {
            let file = sub_m.value_of("file").unwrap().to_string();
            let optional_ext = sub_m.value_of("ext").map(|s| s.to_string());
            let no_backup = sub_m.is_present("no-backup");
            let secure = sub_m.is_present("secure");

            to_plagr(file, optional_ext, no_backup, secure).expect("Failed to convert file");
        }
        ("from_plagr", Some(sub_m)) => {
            let file = sub_m.value_of("file").unwrap().to_string();
            let keys = matches.value_of("keys").map(|s| {
                s.split(',')
                    .map(|part| {
                        let mut split = part.splitn(2, '=');
                        (
                            split.next().unwrap().to_string(),
                            split.next().unwrap().to_string(),
                        )
                    })
                    .collect::<HashMap<String, String>>()
            });
            let no_cleanup = sub_m.is_present("no-cleanup");
            let key_path = sub_m.value_of("key-path").unwrap_or("key.ini");
            let secure = sub_m.is_present("secure");
            let new_ext = sub_m
                .value_of("ext")
                .map(|s| s.strip_prefix('.').unwrap_or(s).to_string());

            from_plagr(file, keys, key_path, no_cleanup, secure, new_ext)
                .expect("Failed to convert file");
        }
        (_, _) => (),
    }
}
