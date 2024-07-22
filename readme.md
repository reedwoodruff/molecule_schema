# Molecule Schema
## Purpose
This project is meant to facilitate the creation and use of a schema for highly contextual data. The assumption is that most or all data can fall into this category of "highly contextual", and would benefit from an explicit modeling in these terms.
For more on purpose, see [a blog post](https://blog.equalityofthought.org/posts/2023-12-06-Starting-A-Devlog)

## Usage Steps
### Create the schema
The first step is to create the schema. This is accomplished through a rudimentary UI contained in the `/molecule_schema` directory.
  - Ensure that you have all of the prerequisites for running a Leptos application (see [the docs](https://book.leptos.dev/getting_started/index.html#hello-world-getting-set-up-for-leptos-csr-development))
  - Install trunk, a tool for building and serving Rust-based web applications. For more information, visit [Trunk's documentation](https://trunkrs.dev/).
  - Navigate to the `/molecule_schema` directory and run the project locally via `trunk serve`.
  - The initial schema contained in `/molecule_schema/resources/schema.json` is populated into the UI. To start fresh, replace `schema.json` contents with the `empty_schema.json` contents and restart the application.
  - Note that this schema-creation UI is quite rough right now. For example, it will crash if you try to delete some construct which is depended upon by another structure.
#### Schema Terminology
  - **Templates** represent the highest-level type. It defines a node which can contain fields and slots.
  - Fields are standalone data associated with the template.
  - Slots are specialized edges which connect to other operatives.
  - **Operatives** are subtypes of templates. They are guaranteed to have all fields and slots of their parent template, but it is possible for these fields and slots to be locked at the operative level.
  - Operatives can be created based on other operatives -- the rule is that they always must become more specialized. Any constraint locked above them in their operative hierarchy cannot be unlocked at a lower level.
  - **Instances** are *entirely* locked operatives. Note that this is not currently enforced in this basic UI, but the assumption is that instances will not be able to be manipulated by the end user, therefore they must have every constraint locked (fields and slots).
  - It must be defined explicitly what kind of nodes can be "slotted" into a given slot. The choices for this selection are: 1) a single, defined operative, or 2) a set of traits which must be fulfilled by some as-yet undefined operative.
  - Slots can be constrained to allow a certain number of edges.

### Save the schema
To save a schema, click "Export Schema" and check the browser console for the JSON version of the schema. Copy and paste this into a json file of your choice.

### Use a build step to create the graph toolkit corresponding to your schema
Include the code-generation in the build step of a project where you want to manipulate the schemaful data.
  - This step generates types, traits, and impls which allow you to create a managed graph environment which is constrained by the schema from which it was generated.
  - Example build script:
  ```Rust
  use std::{env, fs, path::Path};

  use generate_schema_reactive::generate_concrete_schema_reactive;
  fn main() {
      let schema_location = Path::new("/home/reed/dev/molecule_schema/resources/schema.json");
      let out_dir = env::var_os("OUT_DIR").unwrap();
      let dest_path = Path::new(&out_dir).join("schema.rs");

      let final_output = generate_concrete_schema_reactive(schema_location);
      fs::write(&dest_path, final_output).unwrap();
  }
  ```
  - Now `include` the generated code in your project, something like this: (see the build script docs in the [official cargo book](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#code-generation) for a more thorough explanation of this process)
  ```Rust
  include!(concat!(env!("OUT_DIR"), "/schema.rs"));
  ```
  - You now have in scope a managed, reactive graph environment which is typed according to your schema. I'll call this the graph toolkit.

### Build a graph with the toolkit
Use the graph toolkit to build and manipulate instances of your data.
  - Note that this API and the generated method names are subject to change.
  - Note that only *Operatives* are valid nodes in the graph. Any template which one wants to use in practice needs to have a corresponding operative in order to be accessible in the toolkit.
  - Note that currently instances created in the schema are unsupported. The idea is to eventually integrate these schema-defined instances into the graph environment in an efficient way so as not to duplicate data, but this functionality is just a stub right now.
  - Write something like this to build a simple Sentence with a single word with the the default schema:
  ```Rust
    let graph = RBaseGraphEnvironment::<Schema>::new(&CONSTRAINT_SCHEMA);

    let mut editor = Sentence::new(graph.clone());
    editor
        .set_temp_id("parent_sentence")
        .add_new_elements::<Word>(|word| {
            word.set_temp_id("Today")
                .set_display("Today".to_string())
                .add_existing_or_temp_parentsentence("parent_sentence")
        });
    let temp_id_map = editor.execute().unwrap();
    let sentence_id = temp_id_map.get_final_id("parent_sentence").unwrap();

  ```
  - This new sentence now exists in your graph environment, and you can access it using `graph.get(sentence_id).unwrap()`. The sentence can be used in UIs in a reactive way because all fields and slots are stored as reactive signals courtesy of Leptos's signal system. This means you can build a visualization of your data type in such a way that making changes to nested structure is automatically propagated to the UI.
  - You interact with elements in your graph through a structure called the MainBuilder, which is returned from `{operative_name}::new()` or `{instance_of_operative}.edit()`. This structure will have the legal operations according to your schema (e.g. methods to add or remove elements to slots, or to set fields).
  - Call `.execute()` on your RGSOBuilder to attempt to commit the transaction to the graph. If there are no errors, all contained graph operations will be commited, if it fails, none of the operations will occur.
  - The toolkit will error if created elements don't fulfill all of their constraints, or if newly added slotted instances break the schema constraints.
  - Call `graph.undo()` and `graph.redo()` to manipulate your historical transactions.
### (Optional) Connect to Neo4j for visualization.
  - The hope is that this graph toolkit will make it possible to create UIs which allow users to intuitively interact with highly contextual data, but as a nice first step for developers attempting to understand their schemas, Neo4j provides some very nice graph visualization features.
  - A very rough version of generating the requisite Neo4j Cypher query is included in `/resources/neo4j_creation_example.rs`.
  - Examples:
    - ![With sentence structure shown](/resources/semantic_structure.png)
    - ![Without sentence structure shown](/resources/all_structure.png)
