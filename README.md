# diffdirs

A fast, minimal Rust CLI tool for comparing the contents of two directories.
Displays which files or directories exist only in one location or the other,
with optional color output, depth limiting, and JSON support.

<img width="532" height="327" alt="diffdirs" src="https://github.com/user-attachments/assets/11b6e27f-d3ee-4c19-be65-3e81fe5ad900" />

## Features

- Compare two directories for unique paths
- Show **only files** (default) or **only directories** (`--dirs`)
- Limit recursion depth with `--depth`
- Output results as JSON (`--json`)
- Colorized terminal output for readability

## Installation

Build and install from source:

```bash
cargo install --path .
```

or clone and run locally:

```bash
git clone https://github.com/fibnas/diffdirs.git
cd diffdirs
cargo build --release
```

The binary will be available at `target/release/diffdirs`.

## Usage

```bash
diffdirs [OPTIONS] <DIR_A> <DIR_B>
```

### Arguments

| Argument | Description |
| -----------|-------------- |
| `<DIR_A>` | First directory to compare |
| `<DIR_B>` | Second directory to compare |

### Options

| Option | Description |
| --------|-------------- |
| `--dirs` | Compare only directories (not files) |
| `--depth <DEPTH>` | Maximum depth to traverse (`0` = only root) |
| `--json` | Output results in JSON format |
| `-h, --help` | Show help information |

## Examples

Compare two directories (default: files only):

```bash
diffdirs ~/test-a ~/test-b
```

Compare directories only:

```bash
diffdirs ~/test-a ~/test-b --dirs
```

Limit depth to 1:

```bash
diffdirs ~/test-a ~/test-b --depth 1
```

Output as JSON:

```bash
diffdirs ~/test-a ~/test-b --dirs --json
```

Example JSON output:

```json
{
  "only_in_a": [
    "echo",
    "foxtrot"
  ],
  "only_in_b": [
    "bravo",
    "alpha",
    "charlie",
    "delta"
  ],
  "summary": {
    "unique_in_a": 2,
    "unique_in_b": 4
  }
}
```

## Example Output (default mode)

```bash
Only in /home/fn/test-a: file1
Only in /home/fn/test-a: notes.txt
Only in /home/fn/test-b: cheatsheet.txt
Only in /home/fn/test-b: file5

Summary: 5 unique in /home/fn/test-a, 3 unique in /home/fn/test-b
```

## Acknowledgements

> Inspired by earlier directory diff tools such as [diffdir](https://crates.io/crates/diffdir), with a focus on modern CLI ergonomics and flexible output formats.

- [walkdir](https://crates.io/crates/walkdir) for fast recursive directory traversal
- [clap](https://crates.io/crates/clap) for ergonomic CLI argument parsing
- [yansi](https://crates.io/crates/yansi) for lightweight colorized output
- [serde_json](https://crates.io/crates/serde_json) for clean JSON serialization
- Inspiration from classic UNIX `diff` and `tree` utilities

## License

MIT License
See [LICENSE](LICENSE) for details.
