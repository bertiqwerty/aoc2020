import os
import subprocess


def main():
    with open("intro.md", "r") as f:
        intro = f.read()
    cargo_out = subprocess.check_output("cargo run --release").decode("utf-8") 
    with open("README.md", "w") as f:
        f.write(f"{intro}\n```\n{cargo_out}```\n")


if __name__ == "__main__":
    main()
