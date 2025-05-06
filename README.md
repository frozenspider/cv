# CV Maker

Simple and free CV Maker that renders minimalistic CV.

Based on [idimetrix/cv](https://github.com/idimetrix/cv), simplified and ported to Rust.

<details>
<summary>This is what CV looks like:</summary>

![cv-builder](https://github.com/user-attachments/assets/aee25388-2c27-4e33-995e-1718ec529da9)
</details>

All personal information is stored in a single `info.toml` file.
Templates use Jinja syntax and are stored in `templates` folder.

To run the project, use:
```
cargo run
```
This starts an Actix server on http://localhost:3000/ that serves a CV in plain HTML.
This page can be printed to PDF, yielding you a proper CV.

All changes to the `info.toml` and templates are hot reloaded, so you can tweak them without restarting the server.
