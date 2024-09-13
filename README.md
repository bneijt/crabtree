Crabtree: Technically it's Genealogy
==========================

Genealogy software written in Rust with flat data files in TOML format.

Add people in toml format in the `data` folder.

Run without commands will read from the `data` folder and:

- Give every person without an id an id
- Render all id's into a mermaid graph in dist


Getting started
---------------
Copy `doc/example.toml` to `data/data.toml`, run `cargo run -- update` and then load `index.html` in a local webserver.