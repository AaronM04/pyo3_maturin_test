PYO3 Maturin test
=================

To try out using Rust->Python bindings using PYO3 and Maturin.

Main reference: https://pyo3.rs/v0.16.4/#using-rust-from-python

# How I created this

```
cargo new --bin pyo3_maturin_test
cd pyo3_maturin_test
python3 -m venv venv
source venv/bin/activate
pip install maturin
maturin init
# Write some Rust code in src/main.rs
maturin develop
pip freeze > requirements.txt
# Manual edits to remove pyo3_maturin_init and XXX
```

# Setup

```
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

## Recompile

```
maturin develop
```

## Jupyter Notebook support

```
source venv/bin/activate
pip install ipython ipykernel
ipython kernel install --user --name=venv
jupyter-notebook   # Persistent shell
```

Then create a notebook and put this in it:

```
import pyo3_maturin_test
```

Press Ctrl-Enter and there should not be an error displayed. No output is expected.

## Other niceties

```
pip install jedi-language-server   # Vim Jedi LSP (I did ALE but there are other options)
```
