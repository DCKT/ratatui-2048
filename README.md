# game_2048
<img width="1247" height="416" alt="image" src="https://github.com/user-attachments/assets/671f9f76-3911-42b7-a38f-d6925f4abaeb" />

This is a [Ratatui] app that runs both in the terminal and in the browser (via [Ratzilla]).

[Ratatui]: https://ratatui.rs
[Ratzilla]: https://github.com/orhun/ratzilla

## Terminal

```bash
cargo run
```

## Browser

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk serve
```

Then open [http://localhost:8080](http://localhost:8080).

## Tests

```bash
cargo test
```

## License

Copyright (c) Thomas Deconinck <thomas.deconinck@colisweb.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
