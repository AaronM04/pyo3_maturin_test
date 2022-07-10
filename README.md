PYO3 Maturin test
=================

To try out using Rust-to-Python bindings using PYO3 and Maturin.

Main reference: https://pyo3.rs/v0.16.4/#using-rust-from-python

# How I created this

```
cargo new --bin pyo3_maturin_test
cd pyo3_maturin_test
python3 -m venv venv
source venv/bin/activate
python -m pip install pip-tools
# Edit requirements.in
pip-compile --output-file=- > requirements.txt
pip install -r requirements.txt
maturin init -b pyo3                              # Actually the -b is untested
# Write some Rust code in src/lib.rs
maturin develop
```

# Setup

```
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
maturin develop
```

# Recompile

```
maturin develop
```

# Jupyter Notebook support

```
ipython kernel install --user --name=venv
jupyter-notebook   # Persistent shell
```

Then create a notebook and put this in it:

```
import pyo3_maturin_test
```

Press Ctrl-Enter and there should not be an error displayed. No output is expected.

## Edit-Recompile-Run loop in Jupyter Notebook

1. Edit Rust code
2. `maturin develop`
3. In Jupyter Notebook, Kernel -> Restart & Run All
4. Add new code cell(s)
