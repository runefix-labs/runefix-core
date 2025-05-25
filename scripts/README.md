# char-table Integration Script

This script automates the process of downloading and embedding `char-table` Unicode width datasets into the `runefix-rs` project.

## 📦 Script: `scripts/integrate_char_table.py`

### Usage

```bash
python scripts/integrate_char_table.py --version 15.1.0
```

### What it does:

1. Scrapes the GitHub archive listing for the given version. 
2. Downloads the latest matching `.tar.gz` archive. 
3. Extracts the archive to a temporary directory. 
4. Copies all `*.json` files into `src/assets/`. 
5. Updates `CHAR_TABLE_VERSION` in `consts.rs`. 
6. Cleans up all temporary files.

### 🛠 Requirements

- Python `3.13`
- `requests` package
    ```bash
    # Install with:
    pip install -r scripts/requirements.txt
    
    # Or just:
    pip install requests
    ```

### Example

```bash
python scripts/integrate_char_table.py --version 15.1.0
```
After this, commit changes in:
- `src/assets/` (JSON files)
- `src/consts.rs` (version string)

### Notes

- The script only supports versions already published in:\
[runefix-labs/char-table](https://github.com/runefix-labs/char-table/tree/main/char_table/archive)

- This script is not meant to be run frequently—only when upstream width data is updated.
