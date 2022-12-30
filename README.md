# Bim Sort
Bim sort is a command line application to sort Microsoft Analysis Services bim files. 

## Installation

### As an executable
Download the latest version for your operating system from the releases.

### From source
- Install rust and cargo from https://www.rust-lang.org/

- Clone this repository
```bash
git clone https://github.com/girotobial/bim_merge.git
```

- build using cargo
```bash
cargo build --release
```

- Run from the target folder
```bash
./target/release/bim_sort
```

## Usage
---
### From the Commandline / Powershell

To simply sort a bim file.
```bash
bim_sort <the path to your bim file>
```

to display the help page
```bash
bim_sort -h
```

### Using Bim Sort with pre-commit
To use Bim Sort's official pre-commit integration add the following config:

```yaml
-   repo: https://github.com/girotobial/bim_merge
    rev: master
    hooks:
    -   id: bim_sort
        name: bim_sort
```
under the `repos` section of the your `.pre-commit-config.yaml` file.

## Licence
[GNU General Public License version 3](LICENCE.md)

## Project Status
Currently in alpha, looking for testers to validate that all types of bim files can be sorted correctly without errors.