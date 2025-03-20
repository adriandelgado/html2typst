# HTML to Typst Converter

A library for converting HTML documents into Typst markup.

## 🚧 Status: Alpha

This library is currently in **alpha**. Features are incomplete, and the API may change in future versions. Use with caution and report any issues you encounter.

## Features

- Converts basic HTML elements into Typst markup.
- Supports common text formatting elements (e.g., `<p>`, `<b>`, `<i>`, `<h1>`–`<h6>`).

## Usage

```rust
use html2typst::parse_html;

let html = "<h1>Hello, World!</h1><p>This is a test.</p>";
let typst = parse_html(html);

assert_eq!(typst, "= Hello, World!\nThis is a test.");
```

## Limitations

- **Incomplete HTML support**: Only a subset of HTML tags are converted.
- **Experimental API**: The function signatures and output format may change.

## Contributing

Contributions are welcome! If you find a bug or want to add a feature, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
