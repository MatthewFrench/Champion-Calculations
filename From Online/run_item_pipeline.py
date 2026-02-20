import subprocess
import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent
NORMALIZE = BASE / "normalize_items.py"
SAMPLE = BASE / "make_sample_set.py"


def run(cmd):
    print(f"Running: {cmd}")
    subprocess.check_call([sys.executable, str(cmd)])


def main():
    run(NORMALIZE)
    run(SAMPLE)


if __name__ == "__main__":
    main()
