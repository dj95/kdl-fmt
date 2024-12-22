<h1 align="center">kdl-fmt</h1>

<p align="center">
  Formatting CLI for KDL (v1 & v2).
  <br><br>
  <a href="https://github.com/dj95/kdl-fmt/actions/workflows/lint.yml">
    <img alt="clippy check" src="https://github.com/dj95/kdl-fmt/actions/workflows/lint.yml/badge.svg" />
  </a>
  <a href="https://github.com/dj95/kdl-fmt/releases">
    <img alt="latest version" src="https://img.shields.io/github/v/tag/dj95/kdl-fmt.svg?sort=semver" />
  </a>

  <br><br>
  A small formatting utility around the awesome [kdl-rs](https://github.com/kdl-org/kdl-rs) crate for formatting
  KDL files. It is also capable of converting KDL v1 to v2 documents and vice versa.
</p>

## 📦 Requirements

- cargo/Rust

**development dependencies**
- just (for development)
- cargo nextest

## 🚀 Getting Started

Clone the repository and run `cargo build -r`. Then copy the binary from `./target/release/kdl-fmt` somewhere into your `$PATH` (e.g. `/usr/local/bin` on Linux).

Afterwards you can use *kdl-fmt* in your terminal:

```bash
# format the content of file and print the result
$ kdl-fmt ./path/to/file.kdl

# format the content of file and write it back into the file
$ kdl-fmt --in-place ./path/to/file.kdl

# format content from STDIN
$ echo "some { kdl; }" | kdl-fmt -

# convert to v1
$ kdl-fmt --to-v1 ./path/to/file.kdl

# convert to v2
$ kdl-fmt --to-v2 ./path/to/file.kdl

# remove comments
$ kdl-fmt --strip-comments ./path/to/file.kdl

# print help
$ kdl-fmt --help
```

## 🤝 Contributing

If you are missing features or find some annoying bugs please feel free to submit an issue or a bugfix within a pull request :)

## 📝 License

© 2024 Daniel Jankowski

This project is licensed under the MIT license.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
