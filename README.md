# diskusage

Show disk usage sorted by size with human-readable output.

## Install

```console
cargo build --release
sudo cp target/release/diskusage /usr/local/bin/
```

## Usage

```console
diskusage /home/user
diskusage .
```

Output:

```
      5.2G  /home/user
      1.1G  /home/user/.cache
      500M  /home/user/Documents
```
