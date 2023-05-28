# Playground Launcher .plagr Converter

This repository contains a Rust crate that forms part of the Playground Launcher project. The crate provides functionality to convert files to and from the `.plagr` file format, which is a custom file format for our project.

The `.plagr` file format allows us to store different types of files as base64-encoded text files. The original file's extension is stored at the top of the file, making it easy to convert it back to its original format when required.

## New Features

We have migrated the converter to Rust and added some new features. Here's an overview of the changes:

### Encryption (Secure Mode)

You can now encrypt the data before compressing and converting it to `.plagr` format. This feature provides an extra layer of security for sensitive files. To enable encryption, use the `--secure` flag when running the `plagr to_plagr` command.

### Decryption

When converting a `.plagr` file back to its original format, you can now decrypt the data if it was encrypted during conversion. This feature ensures that the data remains secure and can only be accessed with the correct decryption keys. The decryption process requires providing the keys used for encryption and the `--secure` flag to be set. You can either provide the keys as command-line arguments using the `--keys` option or specify a file path for the `key.ini` file using the `--key-path` option.

### File Extension Override

A new feature allows you to pass a file extension to the `from_plagr` command. This feature ignores the file extension set in the `.plagr` file and instead uses the one passed via the command-line argument. This is helpful when you want to ensure a specific file extension for the converted file. To override the file extension, use the `--ext` flag followed by the desired extension (without a leading dot). For example:

```shell
plagr from_plagr test.plagr --ext txt
```

In the command above, the `test.plagr` file will be converted to a `.txt` file regardless of the original file extension stored in the `.plagr file.`

### Backup File Option

The converter provides an option to create a backup of the original file before conversion. This backup file can be useful in case you need to restore the original file later. To create a backup, use the `--no-backup` flag when running the `plagr to_plagr` command.

### Cleanup Option

When converting a `.plagr` file back to its original format, you can choose whether to delete the `.plagr` file and its backup file after conversion. This option provides more control over file management. To skip the cleanup process, use the `--no-cleanup` flag when running the `plagr from_plagr` command.

### Enhanced Error Handling

We have improved error handling in the converter to provide more informative error messages and handle various edge cases.

## Installation

To install the `.plagr` converter, make sure you have Rust and Cargo installed on your system. Then, run the following command:

```shell
cargo install plagr
```

This will install the `plagr` command-line tool, which you can use to convert files to and from the `.plagr` format.

## Usage

To convert a file to `.plagr` format, use the following command:

```shell
plagr to_plagr <file> [--ext <ext>] [--no-backup] [--secure]
```

In the above command, replace `<file>` with the path to the file you want to convert. The `--ext` option allows you to specify a new file extension for the converted file. By default, the original file's extension will be used. The `--no-backup` flag skips creating a backup of the original file, and the `--secure` flag enables encryption.

To convert a `.plagr` file back to its original format, use the following command:

```shell
plagr from_plagr <file> [--keys <keys>] [--key-path <key-path>] [--no-cleanup] [--secure] [--ext <ext>]
```

Replace `<file>` with the path to the `.plagr` file you want to convert. The `--keys` option allows you to provide the decryption keys directly as a comma-separated list in the format `iv_key=<IV_KEY>,key=<KEY>`. Alternatively, you can specify a file path for the key.ini file using the `--key-path` option. The `--no-cleanup` flag skips deleting the .plagr and backup files after conversion, and the `--`secure flag enables decryption, and the `--ext` flag allows you to override the file extension.

> :warning:
**Please note that the `--secure` flag should be used consistently between the to_plagr and from_plagr commands to ensure proper encryption and decryption.**

## Contributing

Contributions are always welcome. If you have any suggestions, improvements, or bug fixes, please open an issue or create a pull request.

## License

This project is licensed under the MIT License - see the `LICENSE` file for more details.

## About the Playground Launcher Project

The Playground Launcher project is a platform designed for managing a variety of applications and projects I create. The `.plagr` file converter is just one piece of the puzzle that enhances our ability to adapt and grow. Stay tuned for more updates and features!
