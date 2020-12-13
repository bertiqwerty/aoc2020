import os
import subprocess


def main():
    intro_file = "intro.md"
    readme_file = "README.md"
    cargo_cmd = "cargo run --release"

    with open(intro_file, "r") as f:
        intro = f.read()
    rust_out = subprocess.check_output(cargo_cmd).decode("utf-8")
    print(rust_out)
    with open(readme_file, "w") as f:
        f.write(f"{intro}\n```\n{rust_out}```\n")


if __name__ == "__main__":
    main()
