# YEngine Python Package

## Local Development

### Build and install locally:
```bash
cargo build --release
cd crates/yengine-bindings/python
python build.py
```

### Test it:
```python
from yengine import Engine

engine = Engine()
engine.run()
engine.shutdown()
```

## Publishing to PyPI

### 1. Install tools:
```bash
pip install build twine
```

### 2. Build package:
```bash
cd crates/yengine-bindings/python
python -m build
```

### 3. Upload to PyPI:
```bash
twine upload dist/*
```

**Note:** You need to build on each platform (Windows/Linux/macOS) to support all users, or use GitHub Actions for automated builds.
