#!/usr/bin/env python3
import subprocess
import sys
from pathlib import Path

def main():
    python_dir = Path(__file__).parent

    print("Installing YEngine Python package...")

    try:
        subprocess.run([sys.executable, "-m", "pip", "install", "-e", "."],
                      cwd=python_dir, check=True)
        print("✓ Package installed locally")
    except subprocess.CalledProcessError:
        print("Failed to install package")
        return 1

    print("✓ Installation completed!")
    print("Note: Make sure to run 'cargo build --release' before using the package.")
    return 0

if __name__ == "__main__":
    exit(main())
