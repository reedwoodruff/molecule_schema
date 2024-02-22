# Dev Log

## Dec 20, 2023

The dissonance is arising from the need to represent the schema in multiple ways at different phases
There needs to be a schema representation which can be fed in its entirety to the graph environment
This schema should contain all possible elements which can be instantiated

But conceptualizing the form of that schema is giving me fits.
There is a concept of an instantiated instance of a schema element, and a concept of a schema element which has not been instantiated

Output of a schema:

1. A library of instantiated elements in a Hashmap<Uid, Instance>
The Uid is unique to this instance, and the Instance should contain information about which schema element the instance is an instantiation of
2. An enum of all possible schema elements
What is the form of this enum? Maybe the element should just be a collection of constraints.
There needs to be some instantiation method which takes a schema element and returns an instance of that element
This is where the constraints come in -- you need to know what constraints are on the element in order to instantiate it

Is there a point to having an enum which contains all of these constraint objects? It seems like the value of an enum
would be its ability to be strongly typed, but the constraints are kind of a DSL
so of what benefit would it be to have a strongly typed enum of constraints?

Maybe I'm missing a third output which splits #2:
2. an enum of all possible schema elements in the form of constraints (to be used for instantiating instances)
3. an enum of instantiated schema elements (to be used as the return type of the instantiation method)

e.g. 2: enum SchemaElements { NodeA(ConstraintContainer(field1: String, field2: String)) }
e.g. 3: enum SchemaInstances { NodeA(NodeA {field1: String, field2: String}) }
(In practice I tihnk there would need to be another enum to facilitate mapping between the two)

I think I'm on to something there.
The question becomes how to handle edge constraints
Or I suppose that #3 could include a copy of the constraints, or some representation of the constraints
The concern is ensuring that constrained edges be treated differently than subsequently added/modified edges

What exactly a "constrained edge" is defined as is somewhat problematic.
As a baseline, I might just say that the only way to constrain an edge is to specify a particular edge as mandatory
(with optional constraints on the target node -- e.g. "this edge must exist, and must be connected to a node of type X")
The primary need for edge constraints arises from templates, where you want to mandate that a particular operative node is slotted.
Would there ever be a case for a node element to be constrained to a particular edge? I kind of think not -- the purpose of a template is to house that complexity.

### Workflow

Suspected issue: The need to reference instantiated schema elements while building the schema
(e.g. in a template, you will likely want to reference instantiated nodes)
How can you instantiate a schema element during the process of creating the schema?

1. Define field types
    Uses macro to regurgitate a Types enum and a map to the values in question
    This allows users to build their field constraints using the Types enum
2. Define schema elements in terms of constraints
    Need to figure out how to build templates in this way -- essentially need to expose a recursive subgraph builder, allowing templates to be composed of templates
    Probably in this step you are also instantiating the necessary nodes, and that might be possible if this is all done in a macro
3. System produces the final outputs, automatically generating :
    - A library of instantiated elements in a Hashmap<Uid, Instance>
    - An enum of all possible schema elements in the form of constraints (to be used for instantiating instances)
    - An enum of instantiated schema elements (to be used as the return type of the instantiation method)


## Jan 9, 2024
Trying now to ascertain what exactly will need to happen in the macro, and what the interface will be between pre-macro types (building the schema with ConstraintObjects) and post-macro types.
It seems like the libraries will need to exist in both places (Instance library & Operative library). For the pre-macro work, you'll need to define templates in terms of instances and operatives, so you'll need to be keeping a running library. For post-macro work, you'll need to reference the library if you want to get information about the structure of a given template.

I'm not certain it makes sense to think of pre-macro work -- maybe it will be intra-macro work. 
The issue I'm grappling with is aligning the input and output of the macro in a way where things work out nicely.

### Brief Overview of Schema Elements
There are two main elements of the schema: Nodes and Templates. Nodes are leaf nodes in the Template tree. Templates can contain other templates or nodes.
After being instantiated, a node or a template is considered an Instance. Instances are totally defined -- all of their fields are filled and any mandatory edges are present.
Before being instantiated, there are several complexities which apply to templates. For a standalone node, instantiation is as simple as filling in its fields and providing any mandatory edges. For templates, we need to introduce the concept of an operative element. For any given template, it may have arbitrarily many operative elements in its constituent structure. These operative elements represent a schema element in a partial state of instantiation.
For example, we have TemplateA, NodeA, and NodeB. TemplateA consists of NodeA totally instantiated, and NodeB in an operative state. What does this mean?
Well if NodeB looks like this {
field1: String,
field2: String
}
then let's say in its operative state as a constituent of Template A, that field1 is filled with "Locked", and field2 is left up to the user's choice.
So when instantiating TemplateA, the instantiator needs to provide a String value for field2, which is conceptually mapped to NodeB's missing field.
Note that NodeB as a standalone schema element is unchanged. Instead, an entry is added into the OperativeLibrary for this particular case. You could imagine other Templates also utilizing an Operative NodeB in their constituent structures, but with a different value for field1, or no value for field1 but some value for field2.
Also worth noting is that an entry is added into the InstanceLibrary for NodeA in our example with TemplateA.

### Interfacing Between Schema Creation Stages
Let me try to give a survey of what I'm thinking right now.

There are kind of 2 stages to schema building:
1. Defining and updating a schema.
  - Inputs: {
    Option<ConstraintSchema>,
    Option<InstanceLibrary>,
    Option<OperativeLibrary>,
  }
  - Outputs: {
    ConstraintSchema,
    InstanceLibrary,
    OperativeLibrary,
  }
2. Conversion of an updated ConstraintSchema into concrete types.
  - Inputs: {
    ConstraintSchema,
    InstanceLibrary,
    OperativeLibrary,
  }
  - Outputs: {
    ConcreteSchema (an enum including a variant for each ConstraintObject in the ConstraintSchema. Also with methods for instantiating new instances of a given object which enforce constraints imposed during the schema definition phase -- e.g. enforcing that a particular edge exists upon creation),
    InstanceLibrary,
    OperativeLibrary,
  }

The problem here is that InstanceLibrary and OperativeLibrary kind of already need to be built with these concrete types
Will I need to create two representations of each library? One for dynamic interaction during phase 1, and one for reference after phase 2?

## Jan 19, 2024
I think that having a kind of trait system is going to make the system much more powerful.
This will be interesting to implement. I believe what I'm going to need to do is separate it into three conceptual parts:
Part 1 will define the traits -- what methods are required in order for a trait to be implemented? What is the return type of those methods?
Part 2 will be describing how a constraint object fulfills a trait. This is somewhat abstract, as I don't think this can be done in terms of methods.
  I believe it will need to be done in terms of internal structure -- a constraint object will define a contract which says, essentially:
  In order to fulfill Trait A, which requires I implement a method A_a which returns type T, I will create a contract which points to the locations I will reach to find type T.
  (Maybe the constraint object has a constituent node which has a particular field "Foo" of type T, so it points to that location as where it will get type T for method A_a)
  This gets a little fuzzy when it comes to how you'd compose a larger type from smaller constituent parts.
  Say method A_a is required to return type Bar {name: String, id: u32}, and you want to get the name from one location in the structure and the id from another.
  Not sure yet how this might work.
Part 3 is actually creating a method from the description provided in step 2. 
  When translating from a constraint object to a SchemaObject, the locations provided need to be translated into a method which reaches out to those locations and returns the values.

This trait system would give a depth to composibility which wouldn't be present in a simple label system which I had been considering before.
    function: Fn<u32> -> u32,
Whether traits or labels, it seems clear that some kind of type-like system is required in order to be able to denote which constructs are allowed to be slotted into a template's dependencies. For simple systems or ones in which you are ok relying on non-deterministic interpretation, not having any types would be ok. However, it quickly becomes apparent that it would be helpful to restrict or type the possible inputs. You could imagine a "performs_action" template where you'd want to restrict one slot to "Entity" and one slot to "Action". Without a system which allows for differentiation, you would be unable to do so but would instead have to accept any construct in any slot. 
The trait system gives a natural kind of inheritance structure, where if a constituent component implements a particular trait and the encompassing template wants to also expose that trait on itself, it should just be able to point to the implementation in the constituent.

It feels more and more like I'm re-implementing Rust inside of Rust. Not quite to that level, but I need to expose some of the powerful compositional patterns.

### Regarding the TTypes and TValues
The ConstraintSchema (and likely the ConcreteSchema) will need to be implemented in terms of some types TTypes and TValues.
TTypes describes the possible field values (for example TTypes::String or TTypes:CustomUserStruct). TValues maps the TTypes variants to concrete types.
In order for this to work, any given constraintSchema needs to keep a related TTypes and TValues.
This will need to be known at the time of parsing the ConstraintSchema from JSON. What is going to be the best way to keep track of these?
Should there be separate JSON files for each which are parsed first into concrete enums, then they could be present for the parsing of the ConstraintSchema?

## Jan 23, 2024
In the process of making a macro which transforms a ConstraintSchema into a concrete schema for use. Encountering questions like: should things like the reactive system be implemented at this stage in the structure of a ConcreteSchema? For example, you could make the fields and edges of a given template be signals. 
Are there ever cases where the step I'm describing (transforming a ConstraintSchema into a usable state) would be desired without including reactive functionality?
I guess I should disambiguate "usable state". By this, I mean several things:
1. The process of creating concrete data structures from the ConstraintSchema -- useful for representing graphs built in schemaful environments
2. The implementation of instantiation and editing methods upon the template structure -- which seems to be useful particularly in an editing environment.
The instantiation and editing methods are where the signals come in handy -- but they require that the data structure be constructed out of signals to support this functionality.
  I could foresee environments which want to be read-only and would not make use of the signal architecture. This doesn't seem too pressing, though -- you'd have all of the same functionality with the signal structure, it would just be a question of performance impact. If this becomes an issue then it ought to be relatively simple to make another, non-reactive representation of the data structures to be used in such contexts.

Current complexity: How to represent and handle operative nodes and edges in templates. 


## Jan 24, 2024
Trying to reason through the instantiation process of a given template as it relates to the operatives contained in its infrastructure.
Currently, I'm thinking there can be two kinds of operatives in a given template's infrastructure.
1. A trait operative, which can be filled by any structure which implements the required trait.
2. A template operative, which defines a more constrained set of requirements.

Worth noting here that part of the goal is to be able to fill either of these operative types with user's choice of:
1. a pre-existing construct, or,
2. a new construct(s) which is co-created upon instantiation

So a trait operative should be conceptually simpler to determine compatibility. Either your existing construct has the trait or it doesn't. If you're creating a new construct, then it just must implement the trait.
A template operative will be a bit more involved. The template operative could be a recursively nested tree of template operatives.
For existing constructs attempting to fulfill the template operative, they would need to be structurally matched against all of the locked elements within the template operative's tree to determine compatibility.
For new constructs, you need to find all of the operative fields within the template operative's tree and prompt the user to fill them out.

There is some complexity here in that this instantiation process might need to know about the graph environment in order to operate as described.
In order to co-create a new construct upon instantiation, the TemplateBuilder would need some kind of hook into the graph environment to create the dependencies before fully instantiating the template in question.
For the case of a pre-existing construct, it could just take the ID of the construct (though for diffing/checking compatibility, you'd need to reach out and check against the construct, which would also require access to the graph environment -- or the instantiator would have to pass in a full copy of the thing manually).
How to accomplish this in a modular way?

### Other thoughts
Thinking through the instantiation story -- it seems likely that one would want to be able to instantiate not only pure templates, but also standalone operative templates.
I've been thinking of operative templates as solely being used as part of the internal structure of higher-level templates, but this is an expansion of that idea.
This gives a mechanism for specializing a particular template without wrapping it in a new template. For example, take a "computer" template. It may have exposed slots for "operating system", "i/o capabilities", "screen size", etc.
Instead of making a new template which wraps "computer" in order to make a reusable "Mac", "Windows", or "Linux" construct, you could instead make an operative instance of "Computer" in which the "operating system" slot is locked to the relevant choice.
The operative instance would live in the operative library. It could then be referenced and instantiated. It also seems useful to be able to implement traits on particular operatives.
For example, while a generic "Computer" template could not implement "Get_Mac_OS_Version" (or something specific like that), the operative "Mac" construct could implement that trait and then be available to slot into slots which require "Get_Mac_OS_Version".
This concept extends to instances as well, and represents a shift from thinking of operatives and instances as *only* serving the template objects to instead being important for the graph environment as well.
This is also a step toward thinking about how real-world/shared-experience things (places, items, people, definitions of concepts, etc.) might be handled.
I've thought that users could add entries into the instance library for things that they want to share access to, though the exact mechanics of this remain elusive.
This would allow users to reference that particular instance without being able to directly modify it (modify in this case referring to its internal structure).

### Other other thoughts
Thinking through how to handle open-ended numbers of operatives. For example, a Sentence template may have a slot for a punctuation mark, and then a slot-like construct for an unlimited amount of words.
Could take a similar approach to Rust's macro_rules and allow slots to be marked with (*, +, ?) which indicate (0 or more, 1 or more, 0 or 1). I can't remember if "?" is the correct symbol.


## Jan 26, 2024
Stepping back a bit to get a bigger picture. I had an insightful conversation with GPT-4 about conceptualizing the schema system, and I want to try to solidify my thoughts.
I had been having difficulty separating the concerns of the schema system from the concerns of creating a graph-based superset of written language, but I think things are becoming a bit clearer.
The schema system's overall goal might be conceptualized as a mechanism for systematically creating what might helpfully be thought of as graph-based Domain Specific Languages (DSLs).
Each individual schema created using the schema building system is its own DSL, which users might use in a graph environment to build a particular "graph" or "document" in the language.
So a kind of "assertion-grammar" schema might be created which is intended to capture the meaning assertions which underpin the meaning inherent in our written/oral linear language.
Using this particular schema, a user could create a document which has a specific meaning corresponding to some linear set of words.
However, the power of the schema-building system is that there might be many other useful DSLs to be created. For example, you could imagine a person-based schema, where people can keep track of facts about themselves and others. It might have things like "name", "email"... Idk, all sorts of things.
The power is when you can start combining the schemas. Ideally, since they're built on the same underlying schema-building primitives, you could reference and make assertions about these People schema-objects using the "assertion-grammar" schema-objects.
What I'm trying to describe is the idea that the schemas and structures within could be incredibly diverse for different domains and use-cases. But this wouldn't stop them from being interoperable (at least that's what I'm hoping -- the specifics aren't nailed down yet).
So instead of continually pointing to our current system of written language as kind of the de-facto way to think about conveying meaning, it instead just becomes a subset of the infinite ways in which the medium might represent meaning.

And really the idea here is that it shouldn't (or at least needn't) revolve around language at all. It could be easy to overfit to our current language system to the detriment of overall flexibility and future expression.
I think that it's probably good to keep this goal of interoperability in mind, but the first step will require just getting a single schema to be able to operate with itself XD.

## Jan 29, 2024
Trying to think of alternative ways to build the schema types without relying on a reference to some kind of graph environment.
Several places seem like they'd require this:
- Resolving trait implementations which rely on data resolved in constituents. 
  I.e. there may be constituents which will need to be initialized before they have the requisite data, so to write a method for the trait implementation, there will need to be some way to retrieve that instantiated constituent.
  E.g. Struct1 is implementing Trait1, which has a method Method1 which returns a u32. Struct1 has an operative constituent Struct2 which has a field "my_num". There's no way at compile time to reference this my_num field directly from Struct1 -- the operative structure of Struct2 has the knowledge of what that field *will* hold, but it will never be filled until it has been instantiated, at which point it is living in the graph environment and not in the schema.
- Instantiation -- whether of a particular ConstraintObject or of the requisite constituent elements required to instantiate that ConstraintObject.

Also grappling with the related question of how to create the final types in a relatively agnostic way, or if that is required. In other words, it seems like it might be helpful to build the types in terms of Signals directly from the macro. I'm not really sure what the alternative would be, other than making a parallel Signaled type and providing some kind of Into<> implementation. 
I just get worried about making the schema macro and the graph environment too coupled. Maybe I shouldn't be, though, as long as the ConstraintSchema remains agnostic.

This feels like it's pushing at the edges of the bigger project picture and I don't know that I have the right answers right now. I might just make a prototype to get something working at a basic level, but I hope to get some insights that make these questions feel like they fit better in to the great whole eventually.

## Feb 5, 2024
Trying to figure out the best way to create a useable user interface for the schema creator/editor.
I think it would be useful to have a kind of tree view, where the selected item was at the top, and its constituents spread out below.
This would facilitate a visual representation of the item in question.
It would also make the experience of implementing traits much more intuitive, hopefully. The idea being that you would:
1. select the top level element which you want to implement a trait on
2. select the method which needs fulfilled
3. select from the child nodes the specific field which will fulfill the method. You could also select a trait impl on a child node which returns the correct value as a fulfiller.
4. voila
Behind the scenes, the path to the field or trait impl could be generated based on the child's position in the tree.

## Feb 7, 2024
Running into some interesting challenges with implementation details regarding the schema constructor.
If I allow users to change the types of fields in ConstraintObjects, then this creates a need for a cascading check to see if that field was used in any trait impls.
Any operatives or instances which had formerly locked a given field with a specific value will have problems if that ConstraintObject field changes out from under them.
Trying to figure out how to address these kinds of issues. There are going to be many interdependent objects in the schema.
It's definitely all theoretically possible, it's a matter of making the user experience of editing templates not too frustrating. The major challenge is in dealing with intermediate states and making sure everything converges to a consistent state.
I think ultimately that it is going to require some robust checking mechanisms -- especially before exporting/finalizing the template. Since a change in one place could have many rippling effects, it would be difficult for the user to make sure that everything is consistent.
Ideally, you could be doing this kind of checking as the edits happen, and then giving visual feedback as to where attention is needed if the changes have caused cascading misalignment.

## Feb 19, 2024
Trying to wrap my head around useful next steps. At a relatively stable place conceptually with the schema editor/creator (though there are still some open questions).
The next big question is regarding how useful it would be to continue with the previous path of creating a macro to generate one large schema enum with methods for instantiation and things.
Under what circumstances would such a thing be useful? What capabilities ought it to have to be most useful?
Maybe taking a step back and away from the macro to consider the goals that are trying to be reached.
The schema creator allows a user to explicitly define a set of rules about what structures are allowed and how they are allowed to interact. It lays the foundation for an environment in which graph constructs can be instantiated according to the rules. 
The next piece of the ecosystem is regarding how that environment will function. We have the foundation (the schema which defines the rules), but the environment is as of yet undefined.
There needs to be a way to instantiate and connect graph constructs according to the schema.
To what end?
Why build these graphs?

The utility and purpose of the graph environment will depend on the schema type and definition.
In the long-term, the ideal would be for all schema environments to be able to interoperate at some level, enabling cross-domain assertions.
But starting out, there will likely need to be domain-specific schemas which capture the domain's details.

It might be difficult to make a general-purpose tool for hosting these graph-environments. The domains/environments could look wildly different.
This is especially difficult before even really having any concrete examples.
Maybe it's jumping the gun to attempt to create a general-purpose tool like this, and the effort ought to go into creating a particular useful schema and an environment which accommodates it.
It would be difficult to do a one-shot attempt at such a schema and have it be correct the first time. The more likely and ideal path is one which allows for iteration and churn in the schema.
And that speaks to a graph environment which is not over-fit on a given schema. In fact, the more loosely coupled, the better.
There are multiple layers of churn to consider. There is churn in the schema-defining language itself (in other words, changes in what makes up a schema and what a schema is allowed to define).
Then there is churn in a given schema definition.
Ideally, churn in the schema-defining language should be kept to a minimum over long periods of time once it reaches stability. Right now it is still in a state of relative flux. It's funny how I can say in the first sentence of this entry that I'm "at a relatively stable place with [... the schema system]", and also say "it's in a state of relative flux", and have both of those things be accurate and relevant in the contexts in which they were generated. It's relatively stable when compared to where it has been and as compared to the state of shadowy projections from which this project was born. It's relatively in flux as concerning API stability.
However, the system ought to be designed to operate without concern for flux in the schemas.

Remaining questions regarding the schema system:
- Accommodation of multiple/unbounded numbers of a given operative (roughly analagous to arrays)
  - Is it necessary? How to implement it?
  - Example use case: modeling the written language domain. Paragraphs have a variable number of sentences. Sentences have a variable number of words.

