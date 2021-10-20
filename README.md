# lingua-py

![CI](https://github.com/messense/lingua-py/workflows/CI/badge.svg)
[![PyPI](https://img.shields.io/pypi/v/linguars.svg)](https://pypi.org/project/linguars)

[lingua-rs](https://github.com/pemistahl/lingua-rs) Python binding. An accurate
natural language detection library, suitable for long and short text alike.

## Installation

```bash
pip install linguars
```

## Usage

```python
import linguars


print(linguars.detect('我们中出了一个叛徒'))
print(linguars.confidence('我们中出了一个叛徒'))
```

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
