# MOLECULE SCHEMA DEVELOPMENT GUIDE

## BUILD & TEST COMMANDS
- Build workspace: `cargo build`
- Run web UI: `cd schema_editor` then `trunk serve` 
- Watch mode: `cargo watch -x check`
- Run tests: `cargo test`
- Run single test: `cargo test test_name`
- Run specific package test: `cargo test -p base_types`
- Lint: `cargo clippy`

## CODE STYLE
- **Imports**: stdlib first, external crates alphabetically, internal modules last
- **Naming**: `snake_case` for functions/variables, `CamelCase` for types/traits
- **Components**: Use `#[component]` attribute with CamelCase names
- **Types**: Explicit annotations for struct fields and function returns
- **Generics**: Use descriptive type parameters (`TTypes`, `TValues`)
- **Error handling**: Use `expect()` with descriptive messages over bare `unwrap()`
- **Formatting**: 4-space indentation, trailing commas in multi-line structures
- **State management**: Use Leptos signals (`RwSignal`) for reactive state
- **Documentation**: Include doc comments for public APIs

## PROJECT ORGANIZATION
- Schema editing UI in `/schema_editor`
- Core type definitions in `/base_types`
- Code generation in `/first_generation_compiler`
- Neo4j visualization support in `/neo4j_pipeline`