import subprocess
import sys

BASE = "/Users/matthewfrench/Documents/League of Legends/Vladimir/From Online"
NORMALIZE = f"{BASE}/normalize_items.py"
SAMPLE = f"{BASE}/make_sample_set.py"


def run(cmd):
    print(f"Running: {cmd}")
    subprocess.check_call([sys.executable, cmd])


def main():
    run(NORMALIZE)
    run(SAMPLE)


if __name__ == "__main__":
    main()
