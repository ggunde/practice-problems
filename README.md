## Practice Problems

This is a collection of practice problems for python and rust programming languages. Each problem is accompanied by a set of tests to help you verify your solution.

### Running python tests

``` bash
# Install uv
pipx install uv

cd ./python

uv venv
uv sync
source .venv/bin/activate

pip install pytest

pytest
```

### Running rust tests

``` bash
# Install rustup and cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cd ./rust/practice-problems

cargo test
```
