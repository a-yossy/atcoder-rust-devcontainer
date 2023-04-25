#!/bin/bash

# This script generates Rust code snippets for Visual Studio Code in the context of Competitive Programming.

# Navigate to the lib directory within the workspace folder
cd $WORKSPACE_FOLDER/src/lib

# Generate Rust code snippets using cargo-snippet, then modify the output using sed to add a scope for Rust,
# and finally, save the modified output to a rust.code-snippets file in the .vscode directory of the workspace folder
cargo snippet -t vscode | sed -r "s/\"prefix\"/\"scope\": \"rust\",\n    \"prefix\"/" > $WORKSPACE_FOLDER/.vscode/rust.code-snippets
