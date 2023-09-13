@echo off

rem Define the paths to your child directories
set trunk_dir=frontend
set cargo_dir=server

rem Start `trunk serve` in the first command prompt window
start cmd /k "cd frontend && trunk serve"

rem Start `cargo watch -x run` in the second command prompt window
start cmd /k "cd server && cargo watch -x run"