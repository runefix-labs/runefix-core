#!/usr/bin/env python3

# === Standard Library ===
import re
import shutil
import tarfile
import argparse
from pathlib import Path
from datetime import date

# === Third-Party ===
import requests


# ========================================
# 📁 Local Project Paths
# ========================================

ROOT = Path(__file__).resolve().parent.parent
CONST_RS_PATH = ROOT / "src" / "consts.rs"
CORE_DATA_DIR = ROOT / "src" / "assets"
SCRIPT_DIR = Path(__file__).parent

# ========================================
# 🌐 GitHub Repository Info
# ========================================

ORG_NAME = "runefix-labs"
REPO_NAME = "char-table"
BRANCH = "main"
ARCHIVE_DIR = "char_table/archive"

# ========================================
# 🔗 Constructed GitHub URLs
# ========================================

ARCHIVE_WEB_URL = f"https://github.com/{ORG_NAME}/{REPO_NAME}/tree/{BRANCH}/{ARCHIVE_DIR}"
ARCHIVE_BASE_RAW = f"https://raw.githubusercontent.com/{ORG_NAME}/{REPO_NAME}/{BRANCH}/{ARCHIVE_DIR}/"


class CharTableIntegrator:
    """
    A class to automate the integration of char-table archives into runefix-rs.
    """

    def __init__(self, version: str):
        self.version = version
        self.archive_name = ""
        self.archive_path = SCRIPT_DIR / f"char_table_auto_v{version}.tar.gz"
        self.extract_dir = SCRIPT_DIR / ".tmp_extract"

    def fetch_archive_name(self):
        """
        Fetch the archive file name that matches the version.
        """
        print(f"🌐 Fetching archive listing for version {self.version} ...")
        resp = requests.get(ARCHIVE_WEB_URL)
        if resp.status_code != 200:
            raise RuntimeError("Failed to fetch archive directory listing.")
        matches = re.findall(r'char_table_[\d\-T_:]+_v' + re.escape(self.version) + r'\.tar\.gz', resp.text)
        if not matches:
            raise FileNotFoundError(f"No archive found for version {self.version}")
        self.archive_name = sorted(matches, reverse=True)[0]
        print(f"📦 Found archive: {self.archive_name}")

    def download_archive(self):
        """
        Download the archive file from GitHub raw URL to a local temp path.
        """
        url = ARCHIVE_BASE_RAW + self.archive_name
        print(f"⬇️ Downloading: {url}")
        r = requests.get(url, stream=True)
        if r.status_code != 200:
            raise RuntimeError("Failed to download archive.")
        with open(self.archive_path, "wb") as f:
            for chunk in r.iter_content(chunk_size=8192):
                f.write(chunk)

    def extract_archive(self):
        """
        Extract the archive into a temp directory.
        """
        print(f"🗂️ Extracting: {self.archive_path.name}")
        if self.extract_dir.exists():
            shutil.rmtree(self.extract_dir)
        self.extract_dir.mkdir()
        with tarfile.open(self.archive_path, "r:gz") as tar:
            tar.extractall(path=self.extract_dir, filter="data")

    def copy_json_data(self):
        """
        Copy valid .json files into the project assets directory.
        """
        current_dir = self.extract_dir / "char_table" / "current"
        if not current_dir.exists():
            raise FileNotFoundError("Missing `current/` directory in the archive.")
        CORE_DATA_DIR.mkdir(parents=True, exist_ok=True)

        count = 0
        for file in current_dir.rglob("*.json"):
            if file.name.startswith("._") or file.name == ".DS_Store":
                continue
            target = CORE_DATA_DIR / file.name
            shutil.copy(file, target)
            print(f"✅ Copied {file.name}")
            count += 1
        return count

    def update_consts_rs(self):
        """
        Update or insert the UNICODE_VERSION in consts.rs.
        """
        today = date.today().isoformat()
        print(f"🛠️ Updating UNICODE_VERSION in consts.rs → {self.version} @ {today}")
        content = CONST_RS_PATH.read_text()

        lines = content.splitlines()
        new_lines = []
        replaced = False

        for i in range(len(lines)):
            if "pub const UNICODE_VERSION: (u8, u8, u8)" in lines[i]:
                new_lines[-2:] = [  # Replace previous comment and const
                    f'/// Unicode Version used by this build (auto-synced).',
                    f'/// auto-updated: {today}',
                    f'pub const UNICODE_VERSION: (u8, u8, u8) = ({", ".join(self.version.split("."))});'
                ]
                replaced = True
            else:
                new_lines.append(lines[i])

        if not replaced:
            # If not found, add at the end
            new_lines.append("")
            new_lines.append(f'/// Unicode Version used by this build (auto-synced).')
            new_lines.append(f'/// auto-updated: {today}')
            new_lines.append(f'pub const UNICODE_VERSION: (u8, u8, u8) = ({", ".join(self.version.split("."))});')

        CONST_RS_PATH.write_text("\n".join(new_lines) + "\n")

    def clean_up(self):
        """
        Clean up archive and temporary extract directory.
        """
        if self.archive_path.exists():
            self.archive_path.unlink()
        if self.extract_dir.exists():
            shutil.rmtree(self.extract_dir)

    def run(self):
        """
        Execute full integration process.
        """
        self.fetch_archive_name()
        self.download_archive()
        self.extract_archive()
        self.copy_json_data()
        self.update_consts_rs()
        self.clean_up()
        print(f"\n🎉 Integration complete! Unicode Version v{self.version} is now embedded.\n")


def main():
    parser = argparse.ArgumentParser(description="Integrate char-table into runefix-rs.")
    parser.add_argument("--version", required=True, help="Target Unicode Version, e.g., 15.1.0")
    args = parser.parse_args()

    integrator = CharTableIntegrator(args.version)
    integrator.run()


if __name__ == "__main__":
    main()
