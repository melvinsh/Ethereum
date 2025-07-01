# Ethereum Vanity Wallet Generator

![carbon](https://github.com/user-attachments/assets/145a42af-ee7e-4b56-aae7-d2ad424d04c1)

A fast, multi-threaded Ethereum wallet generator with two modes:
- **Prefix mode**: Find addresses with a specific prefix.
- **Clean mode**: Find addresses with a high ratio of numbers or letters.

## Features
- Multi-core CPU support for fast searching
- Colored, readable output for matches
- Graceful shutdown with Ctrl+C
- Configurable search parameters

## Usage

### Build

```
cargo build --release
```

Or use the provided script:

```
./run.sh [arguments]
```

### Run

#### 1. Prefix Mode (default)
Find addresses starting with a given prefix (default: `0x1337`).

```
./run.sh --prefix 0x1234
```

#### 2. Clean Mode
Find addresses where at least a certain percentage of the address is either numbers or letters (default: 70%).

```
./run.sh --clean
```

##### Set the Clean Ratio
You can set the threshold (as a float between 0.0 and 1.0):

```
./run.sh --clean --clean-ratio 0.9
```
This will only show addresses where at least 90% of the characters are numbers or letters.

#### 3. Help
Show all options:

```
./run.sh --help
```

## Parameters
- `--prefix <prefix>`: Find addresses with this prefix (default: `0x1337`).
- `--clean`: Enable clean mode (ignore prefix).
- `--clean-ratio <float>`: Ratio threshold for clean mode (default: `0.7`).
- `--help`: Show usage information.

## Output
- Matches are printed in color with address and mnemonic.
- Progress is shown as `Wallets checked: ...` on a single updating line.
- Press Ctrl+C to stop and see summary statistics.

## Dependencies
- [bip39](https://crates.io/crates/bip39)
- [hdwallet](https://crates.io/crates/hdwallet)
- [k256](https://crates.io/crates/k256)
- [tiny-keccak](https://crates.io/crates/tiny-keccak)
- [hex](https://crates.io/crates/hex)
- [rayon](https://crates.io/crates/rayon)
- [num_cpus](https://crates.io/crates/num_cpus)
- [rand](https://crates.io/crates/rand)
- [colored](https://crates.io/crates/colored)
- [ctrlc](https://crates.io/crates/ctrlc)

## Example
```
./run.sh --prefix 0xdead
./run.sh --clean --clean-ratio 0.8
``` 
