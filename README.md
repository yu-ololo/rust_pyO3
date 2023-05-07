# rust_pyO3
## How To Use
```
poetry add maturin
rustup update
poetry run  maturin init
✔ 🤷 Which kind of bindings to use?
  📖 Documentation: https://maturin.rs/bindings.html · pyo3
```

## build
```
poetry run maturin build -i python3 --release
```
 dir構成でハマった。poetry + pyO3で実装するときは、どうするのがいいんだろう？

## python パッケージ

```
maturin develop --release
```
