# Rustify

Rustify is a fast and simple command-line tool that **sorts and organizes files** in a given directory  
based on file **extensions** and predefined **categories** (e.g., images, documents, audio, video, etc).

Built with [Rust](https://www.rust-lang.org/), using `clap`, `walkdir`, and `std::fs`.

---

## 📦 Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/ixand/rustify.git
   cd rustify
   ```

2. **Build the project**:

   ```bash
   cargo build --release
   ```

3. **Run the binary**:

   ```bash
   ./target/release/rustify /path/to/your/folder
   ```

---

## 🚀 Usage

```bash
rustify <DIRECTORY> [OPTIONS]
```

* `DIRECTORY`: The path to the folder you want to organize.
* `OPTIONS`:

  * `--dry-run`: Show what would be moved, without making changes.

### Examples

```bash
rustify ~/Downloads
rustify ~/Downloads --dry-run
```

This will scan your `Downloads` folder and move files into subfolders like:

```
Downloads/
├── Images/
├── Documents/
├── Audio/
├── Video/
├── Archives/
├── Code/
├── Other/
```

Each category is based on file extensions.

---

## 🧠 How It Works

Rustify uses predefined file extension mappings to group files into categories:

* `.jpg`, `.png`, `.gif` → `Images/`
* `.pdf`, `.docx`, `.txt` → `Documents/`
* `.mp3`, `.wav` → `Audio/`
* `.mp4`, `.avi` → `Video/`
* `.zip`, `.tar.gz` → `Archives/`
* `.rs`, `.py`, `.js` → `Code/`
* Unknown types → `Other/`

---

## 🛠 Features

* Cross-platform (Linux, Windows)
* Fast and lightweight
* Safe: creates folders and moves files without deleting anything
* Supports **dry-run mode** (`--dry-run`): preview what will be moved without making any changes
* Open source

---

## 🔮 Future Plans

* Configurable file type mappings (`config.toml`)
* Recursive subfolder support (optional)
* Logging and reporting

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

## 🙌 Contributions

Feel free to open issues or submit pull requests to improve Rustify!


