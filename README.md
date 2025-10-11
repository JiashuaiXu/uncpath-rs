# uncpath

A command-line utility to convert UNC paths to POSIX paths based on a configurable mapping table.

## Features

- **Multiple UNC Format Support**: Recognizes and converts paths in the following formats:
  - Windows UNC: `\\host\share\path`
  - Unix-style: `//host/share/path`
  - SMB URL: `smb://host/share/path`

- **Flexible Mapping Configuration**:
  - Default mappings for common use cases
  - Environment variable support (`UNCPATH_MAPPINGS`)
  - Load mappings from JSON files
  - Command-line mapping overrides

- **Case-Insensitive Matching**: Host and share names are matched case-insensitively for convenience.

## Installation

### From Source

```bash
git clone https://github.com/JiashuaiXu/uncpath.git
cd uncpath
cargo build --release
# Binary will be at target/release/uncpath
```

### Using Cargo

```bash
cargo install --path .
```

## Usage

### Basic Usage

Convert a UNC path using default mappings:

```bash
uncpath '\\server\shared\documents\file.txt'
# Output: /mnt/shared/documents/file.txt
```

### Custom Mappings

Add custom mappings via command line:

```bash
uncpath --mapping 'myhost:myshare:/custom/mount' '\\myhost\myshare\test.txt'
# Output: /custom/mount/test.txt
```

Multiple mappings can be specified:

```bash
uncpath \
  --mapping 'host1:share1:/mount1' \
  --mapping 'host2:share2:/mount2' \
  '\\host1\share1\file.txt'
```

### Load Mappings from File

Create a JSON file with your mappings:

```json
[
  {
    "host": "server",
    "share": "shared",
    "mount_point": "/mnt/shared"
  },
  {
    "host": "nas",
    "share": "backup",
    "mount_point": "/mnt/backup"
  }
]
```

Then use it:

```bash
uncpath --file mappings.json '\\server\shared\file.txt'
```

### Environment Variable

Set the `UNCPATH_MAPPINGS` environment variable with a JSON array:

```bash
export UNCPATH_MAPPINGS='[{"host":"server","share":"shared","mount_point":"/mnt/shared"}]'
uncpath '\\server\shared\file.txt'
```

### List Configured Mappings

View all configured mappings:

```bash
uncpath --list dummy
# Output:
# Configured mappings:
#   \\server\shared -> /mnt/shared
#   \\nas\data -> /mnt/nas
```

### Skip Default Mappings

Use only your custom mappings:

```bash
uncpath --no-defaults --mapping 'host:share:/mount' '\\host\share\file.txt'
```

## Default Mappings

The following default mappings are included:

- `\\server\shared` → `/mnt/shared`
- `\\nas\data` → `/mnt/nas`

Use `--no-defaults` to skip these.

## Examples

### Windows UNC Path

```bash
uncpath '\\server\shared\documents\report.docx'
# Output: /mnt/shared/documents/report.docx
```

### Unix-style Path

```bash
uncpath '//nas/data/backup/archive.tar.gz'
# Output: /mnt/nas/backup/archive.tar.gz
```

### SMB URL

```bash
uncpath 'smb://server/shared/media/video.mp4'
# Output: /mnt/shared/media/video.mp4
```

## Command-Line Options

```
Usage: uncpath [OPTIONS] <PATH>

Arguments:
  <PATH>  UNC path to convert

Options:
  -m, --mapping <MAPPING>  Add custom mapping (format: host:share:mount_point)
  -f, --file <FILE>        Load mappings from JSON file
      --no-defaults        Skip default mappings
  -l, --list               List all configured mappings
  -h, --help               Print help
  -V, --version            Print version
```

## Configuration Priority

Mappings are loaded in the following order (later sources override earlier ones):

1. Default mappings (unless `--no-defaults` is used)
2. Environment variable (`UNCPATH_MAPPINGS`)
3. File (`--file`)
4. Command-line mappings (`--mapping`)

## Building and Testing

```bash
# Build
cargo build

# Run tests
cargo test

# Build release version
cargo build --release
```

## License

MIT License - See [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.