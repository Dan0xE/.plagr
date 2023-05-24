# Playground Launcher .plagr Converter

This repository contains a Python script that forms part of the Playground Launcher project. The script is designed to convert any file into a `.plagr` file format, which is a custom file format for our project.

The `.plagr` file format allows us to store different types of files as base64 encoded text files. The original file's extension is stored at the top of the file, making it easy to convert it back to its original format when required.

---

## Why .plagr?

The `.plagr` file format provides a unified approach to handling different file types. By storing files as base64 encoded text, we can seamlessly transfer and decode them when necessary. The original file's extension is stored at the top of each `.plagr` file, making it straightforward to convert it back to its original format.

By adopting the `.plagr` file format, we also enhance the customizability of software installation. It also facilitates the process of fetching releases from our GitHub repositories, ensuring smooth and efficient updates to our software (like nevo).

---

## Usage

The Python script `to_plagr.py` is designed to convert files of any format into `.plagr` files. It's usage is as follows:

**Convert a file to .plagr format:**
```shell
python to_plagr.py myfile.txt --ext .jpg
```

In the above example, `myfile.txt` is converted to a `.plagr` file. When this `.plagr` file is converted back to its original format, it will have a `.jpg` extension. A backup copy of myfile.txt (myfile.txt.bak) will be created.

If you don't want to create a backup of the original file, you can include the `--no-backup` flag:

```shell
python to_plagr.py myfile.txt --ext .jpg --no-backup
```

If the `--ext` argument is omitted, the original file's extension will be used. For example:

```shell
python to_plagr.py myfile.txt
```

This command will create a `.plagr` file from `myfile.txt`. When this `.plagr` file is converted back to its original format, it will have a `.txt` extension.

**Convert a .plagr file back to its original format:**

```shell
python from_plagr.py myfile.plagr
```

In this example, `myfile.plagr` is converted back to its original format. Both `myfile.plagr` and the backup file (`myfile.txt.bak`) will be deleted.

If you don't want to delete the `.plagr` and `.bak` files, you can include the `--no-cleanup` flag:

```shell
python from_plagr.py myfile.plagr --no-cleanup
```

---

## Installation

First, clone this repository to your local machine using git:

```shell
git clone https://github.com/dan0xe/plagr.git
```

Now you can run the script as shown in the usage section above.

---

## Contributing

Contributions are always welcome. Please feel free to open an issue or create a pull request if you have any changes you'd like to suggest.

---

## License

This project is licensed under the MIT License - see the `LICENSE` file for more details.

---

## About the Playground Launcher Project

The Playground Launcher project is a platform designed for managing a variety of applications and projects i create. As the project continues to evolve, the `.plagr` file converter is just one piece of the puzzle that enhances our ability to adapt and grow. Stay tuned for more updates and features!
