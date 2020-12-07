import os


def main():
    tmp_file = "tmp_res.txt"
    os.system(f"cargo run --release > {tmp_file}")
    with open("intro.md", "r") as f:
        intro = f.read()
    with open(tmp_file, "r") as f:
        res = f.read()
    with open("README.md", "w") as f:
        f.write(f"{intro}\n```{res}```\n")


if __name__ == "__main__":
    main()
