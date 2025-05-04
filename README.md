# CV Maker

Simple and free CV Maker that renders minimalistic CV.

Based on [idimetrix/cv](https://github.com/idimetrix/cv), simplified and ported to Rust.

Running the project starts an Actix server that serves a CV in plain HTML on
[localhost::3000](http://localhost:3000). This page can be printed to PDF, yielding you a proper CV.

All personal information is stored in a single `info.toml` file.
Templates use Jinja syntax and are stored in `templates` folder.

All changes to the `info.toml` and templates are hot reloaded, so you can tweak them without restarting the server.
