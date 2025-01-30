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

## Feb 22, 2024
Still a bit murky how it will be most useful to expose the final generated schema. Maybe it would be best to expose a construct which essentially encapsulates an environment based on that schema.
For example, it could expose all of the graph functionality you'd need to populate the environment according to the given schema.
The other option would be to allow the user to bring their own environment, which was kind of what I was planning.
The benefit of bundling in the environment would be a cleaner interface for the user. The downside would be that they'd be stuck with my novice graph implementation.
Would this mean making the output types reactive? I could still see a case for applications on the backend wanting access to non-reactive versions of the types.
And to communicate with these systems, it would be nice to create mappings between reactive versions on the frontend to non-reactive versions.

Specifically, this is coming up because I'm running into the necessity for a way to store and retreive locked fields/values of operative elements (and all fields/values of instances).
I'm not totally sure how to do this while keeping the separation of concerns I was aiming before.
If I go the route of encapsulating everything within an environment, then I could store this information efficiently in a map structure and just return the values from there instead of instantiating new versions of them every time an instance or operative is referenced.

It seems like there's some misalignment in the goals. The encapsulated environment lends itself to a dynamic, reactive application. The decoupled environment lends itself to non-reactive types. I'm having trouble really getting a good grasp on the purposes of these different environments and what functionalities would need to be exposed in each.

## Feb 24, 2024
Trying to decide how important it is to support custom data types in template fields. Right now it's hard-coded to have a few primitive options (Strings, integers, floats, etc.).
Would it be helpful to allow user-created compound types as a value for a single field? Probably, though it's not terribly clear to me right now how.
In some sense, that seems to be what having an operative or instance constituent accomplishes -- a way to have an arbitrarily nested data type associated with the construct.
That thought represents a breakthrough, in some sense. Thinking about the graph as a kind of evolving type. The structure which a path takes between two elements represents a kind of additive contextual edge, providing a new layer of meaning to the snapshot of the path at that time.

## Feb 27, 2024
Rules for instances to fulfill an operative slot (A):
- The instance is directly an instance of the operative A.
- The instance is an instance of an operative (B) which contains operative A in its ancestors. This would mean that the operative B might be more specialized than operative A, but upholds the contract which operative A represents.

A question arises regarding whether and how to support the following situation:
There exists a template A
An operative B is made from template A, and several fields or constituents are locked in the operative.
An instance C is made from template A. The instance is manually set to fulfill all of the same locked fields and constituents as operative B.
So at this point, instance C is in some sense compatible with operative B even though B does not exist in C's ancestors. They share a common root template A, and just happen in this moment to share matching data for all required locked specifications.
The question really breaks out into two separate situations:
1. The schema building process
2. The graph building process in an environment based on a fixed schema.

It seems important to support some semblance of this functionality in situation 2. Say for example that you have a template "Fireperson" which requires a red-haired person for a particular operative slot. Your schema exposes a "RedHairedPerson" which is an operative based on the template "Person", in which the hair field is locked to red.
During the course of using the graph environment, a Person "Bob" is created based on just the "Person" template. Bob's hair happens to be red. When creating a new "FirePerson", it seems like the user should be able to fill in the FirePerson's "RedHairedPerson" slot with Bob, even though Bob was not instantiated based on that specialized operative, but rather with the generic "Person" template, because the contract in question is fulfilled.
This brings up concerns regarding changes to elements in such situations. Since there is no contract that a "Person" *must* have red hair, a user could go in and change Bob to have brown hair, and then there would be a conflict with the newly created FirePerson since Bob no longer fulfills its operative contract.
Maybe there could be a process where by slotting an instance into an operative slot which it happens to fulfill at the moment, it could essentially subscribe itself to the operative contract in question? So once Bob was used in a FirePerson (or at least *while* he is being used in a FirePerson), then his instance is actually an instance of a RedHairedPerson instead of just a Person. So users would not be able to change his hair.

In situation 1, the question is less pronounced in the sense that once you have a schema that correctly fulfills all of its contracts (regardless of ancestor heirarchy), then once you export that schema you can never change the library instances. But if you take into account the schema-editing process, then you have the exact same problem as in the scenario just discussed. In order to make sure that your schema is consistent at the end, you'd have to have some mechanism in place to make sure that instances being used to fulfill particular operative contracts can't change out from under you.
Related to this is the opposite problem. Instead of the instance changing out from under the contract, there exists the possibility while editing a schema to have the *contract* change out from under the instance.

This is mostly a usability concern -- as far as correctness of the system, you could just force users to manually create and assign the exact operative contract required. But that would be a pretty big UI drag I think.


## March 19, 2024
Confronted with the question: how much of a functional programming language should be implemented into the schema system?
With the addition of operative slots having variable numbers of elements, there needs to be some more expressive way of propagating trait implementations through the constituent structure.
Previously, you could select a field or trait impl of some constituent operative and be sure that there would be exactly 1 instance from which to extract the information required.

Longer term, it seems like it would be very valuable to be able to express some kind of map or filter operation on the elements of a given operative slot.
But this would require creating a whole syntax for mapping over the structure -- some way to:
1. express the operations which would be performed on the elements,
2. perform arithmetic to combine or otherwise manipulate the results of the operations,
3. massage the results into the correct shape to fulfill the expected trait contract.

This seems difficult. I'm searching for a stopgap solution in the meantime.
Maybe it would be best to only allow trait propagation through 1:1 operative slots for now.


## April 10, 2024
Trying to nail down a version of the builder pattern which would be serviceable.
Ideally you could start creating some new Template (say a Sentence), and fluently chain the creation of prerequisite constituent operatives.
You should be able to either provide existing nodes to slot in or create new ones as necessary.
I'd really prefer an interface something like this:
let new_sentence =
    Sentence::new()
    .add_new_word()
      .set_word_value("Word1")
      .build()                    // This building the new word and adding it to the sentence's operative slot.
    .add_existing_word_by_id(42)
    .build();                     // This building the sentence.

Any subnode created for a slot should expose the same interface as if it was being built standalone, and then just return to the higher-order builder when it is finalized.

## April 15, 2024
Difficult at this point to see how/if everything is going to come together. There are many unanswered questions and unasked questions. Right now, it seems like the prudent thing to do is to get a rough working implementation of the graph environment up which interfaces with the schema system. This would allow for some experimentation with different schemas to see how useful/unuseful the entire system is. It's surprisingly difficult to conceptualize how the resulting graphs will behave or speculate about their properties (largely, I think, because that is more a function of the specific schema than it is of the overarching system). That, I suppose, was the impetus for the whole system -- the hope that usefulness beyond what can immediately be conceptualized will be unlocked by providing an environment in which one could easily manipulate and interact with a schemaful graph.
The hierarchy of written language is a good starting point -- encoding documents, paragraphs, sentences, and words. The hope is that alongside this structural schema could exist some more semantic schema. A rich text editor could use the structural schema to lay out the words how we're used to seeing them, and then there would need to be some creative way of visualizing and interacting with the semantic schema.

## April 18, 2024
Coming to a better understanding that I think the key primitives in any standard library schema should actually be the relationships rather than the objects.
In other words, rather than trying to model a bunch of abstract entities which various things can be loosely represented by, instead trying to abstract out the way things relate to each other. The idea is that entities or complex state could then be modeled as arbitrarily large collections of these primitive associations.

It's not clear that there won't need to be *some* kind of entity representations. I think the point here is more in regards to the direction of specificity flow.
As things become more specialized, they can kind of encapsulate the less-specific things, rather than the other way around.
Instead of having a "building" entity concept which is complex enough to house every single type of building, instead you have a "building" entity which is very basic and consists only of the concepts which are integral to "buildingness" -- maybe location, for example. And then "library" might have "building" in its constituent structure (perhaps connected to by a "consists_of" relationship) but it could also include many other constituent concepts in parallel.

There is a helpful interaction with the trait system here, I think. For example, you could have a "building" entity and also a "building" trait. The building entity would implement the building trait.
Anything that has a "building" in its constituent structure could essentially "lift" the building trait so that it also satisfies the building trait.

Truth is, I'm not sure it's super helpful to delineate between entities and relationships right now. It seems like they kind of mix together, as will hopefully become more apparent. What we'd think of "relationships" might actually be a conglomerate subgraph consisting of various things (some of which we might classically consider entities, and some which we might classically consider relationships). And the same would be true for things we'd normally call "entities" (they'd be composed of a conglomeration of entities and relationships).
For example, you might have several a very basic relationships like "consists_of". Then you might specialize that to make a new relationship "has_color", which is essentially a "consists_of" with the stipulation that the pointed-to thing be a color.
And then a cardinal bird might consist of some "bird" subgraph and a "has_color" relationship with red.

There's still a lot of thought to put into this, but it seems apparent to me that it will be required to have a very lean set of basic "relationships" or "assertions" which serve as building blocks for more complex meaning.

Base assertion candidates:
- relates_to (it might be useful to have a totally neutral base assertion like this which could be used with qualifiers to build new meaning. In fact, maybe all other assertions should ultimately be built with this one)
- consists_of
- precedes/succeeds (denoting linear order)

Base entity candidates:
- entity (again, a neutral base entity which serves as the building block)

## April 25, 2024
I am really loathe to maintain reactive and non-reactive versions of everything, but that's the route I've opted for right now. Maybe at some point I could write a macro to annotate non-reactive structs to create reactive versions, but I think it would still be required to manually update any functionality by hand. Idk, I hope there's a better solution eventually, as this adds serious friction to changing and improving things over time as the changes need to be mirrored multiple times.

## May 1, 2024
Thinking through the interface for interacting with various kinds of slots.
Two kinds of slots exist:
- library-operative-specific (can only hold instances of a particular library operative)
- trait-specific (can hold any instances which conform to its trait bounds)

For library-operatiave-specific, the question is arising how to handle inheritance/subclasses.
For example, if you have a slot which accepts an operative Animal, and a subclass operative "DogAnimal" is created based on the Animal operative (which must mean that Dog *at least* fulfills every contract which Animal does, but that it *could* be made more specific by locking a particular field or by locking instances into slots)
The current implementation is looking for the particular struct created for Animal, but I think a more flexible solution is in order.
I'm considering creating an enum for each case when multiple operatives are acceptable.
This would be helpful for when you are attempting to access a member of a given slot. Since a slot which can hold Animal can also hold any subclass of animal, you can't just assume that the node in question is Animal. Instead you could return an enum of all subclasses of Animal.
This seems to have the potential to get wildly out of hand. For example, Animal could have multiple subclasses (e.g. DogAnimal, BirdAnimal), and then those subclasses could have subclasses (e.g. BirdAnimal: SparrowBirdAnimal).
So if you had a slot which returns an Animal, you'd actually be returning an enum of all nodes in a potentially deep tree.

On the other hand, this specialization functionality is ill-suited to this kind of usage. It seems like what I described above would be better suited to using an "Animal" trait.
However there's nothing stopping a schema author from falling into this specialization trap.

The case seems more straightforward for applying this same solution to trait-specific slots.
You could have an enum which has members that correspond to all operatives which implement the given set of traits.
You could then also implement the trait on the enum itself, saving the end user from having to match on a potentially large list if they only needed the functionality provided by the trait or set of traits.

I think it would not be possible in the library-operative-specific case to fully carry across this nice user experience.
For example, with Animal, you might have a field "name" which is unlocked and therefore you have a method which can set it.
But with SparrowBirdAnimal, you might have locked this field.
So there's not really any way to generalize over all of the methods even though they all share an ancestor.
It might be possible to look into providing a subset of those methods (for example, all of the get-related ones) on the resultant enum.
But in order to make any changes you'd need to do a deep match.

Once again, though, I think that that pattern is antipattern. The idea should be to build in a composable fashion. The details of how to do that in this case escape me at the present.

## July 24, 2024
It seems like it should be possible to check the validity of graph structures at compile time.
I'd like to think through the necessary typestate which could achieve this goal.
High level goals:
- Only expose methods for adding fulfilling elements to fields or slots when these are valid actions (i.e. don't allow the user to call "add_element_to_slot" if that slot's capacity is full)
- Similarly, only expose methods for removing from a slot or field when that is a valid action.
- Only allow the user to build/finalize the structure if all fields and slots are in a satisfied state

Currently, it seems like there is only one conceptual bottleneck in making the graph-construction process entirely compile-time safe: adding an existing element to a slot.
At this moment, I'm having a difficult time seeing how to perform the necessary check to see if the element is the correct type. Right now, this interface
just accepts an ID and performs the check at runtime. Perhaps it would be possible to alter the interface.

## August 20, 2024
The typestate system is coming along more or less as envisioned (to my surprise).
I am reaching a point where it would be good to settle on an interface for interacting with the graph builder.
I am thinking through the editing story and where to store the necessary typestate, but it would be beneficial to hone the interaction surface before finalizing anything.
Right now there are two entry points:
- Calling OperativeName::new()
- Calling ExistingOperative::edit()
Both of these entry points return a FreshBuilder which represents some subgraph (or potentially multiple disconnected subgraphs if MainBuilders are combined with `integrate`)
Once the user has a FreshBuilder, they have 3 choices for filling a given slot:
- add_new_{slot_name}_{slotted_operative_name}
- add_existing_or_temp_{slot_name}_{slotted_operative_name}
- add_and_edit_existing_{slot_name}_{slotted_operative_name}
This is somewhat confusing as an interface, I think. I'd like to make it more intuitive.
Previously, each of these methods could require a type parameter to specify which type of operative was being slotted into the slot
  (if there was only one possible fitting operative in the whole schema, then this type parameter was not exposed to the user)
With this typestate rewrite, though, it became convenient to enumerate that type parameter into as many methods as necessary to cover all of the legal operatives.
  (Since the method was now accepting several additional type parameters corresponding to typestate, asking the user to specify one of the type parameters meant
  that the compiler would complain unless they specified all of them, so in practice the method calls started looking like this:
  `add_new_{slot_name}::<DesiredOperative, _, _, _, _>::(...)`
  which is worse than the alternative, I think. Still feeling this one out, though.)

One of the main questions is regarding how to handle referencing existing nodes to add to a slot (or to edit).
Currently there is a lot of runtime checking done to ensure that the id passed into one of these methods points to a valid item for the operation in question.
This wasn't too big of a deal earlier since I was already doing a bunch of runtime checking to ensure that all of the operations in a final FreshBuilder adhered to the schema.
But now, I think a lot of this checking is redundant with these typestate guards. The typestate should (in theory) make it impossible for a user to misconfigure any action.
The only remaining runtime check which seems to be necessary is that of referencing an existing node (or a temporary node, which is also a fallible action if the user did not create a temporary node of the same name that they are subsequently looking up).
It seems to me this should be done explicitly immediately upon calling a method which requires an existing node, and that method should return a Result.
But! This would require then that every builder method would need to return a Result since all of these methods can be arbitrarily nested. What a mess.

Ai had the good idea to perform the checking in a separate step. So you could have a method which returns a type which essentially says "this is guaranteed to exist and be of this type"
This would work well for nodes which existed before the builder came into existence, but it would be a bit trickier for "temporary" nodes -- nodes which were created within the same builder session.
There is still the question of invalidating these "guarantee" types when/if another builder is executed in between creation and usage.

## October 7, 2024
Can't figure out how to do fully compile-time safe operations. Editing an existing graph is proving to be beyond my reach.
The issue lies with maintaining the typestate of all current nodes. When you go to edit a node, you need to know how many items it has slotted in each slot so as to create a builder with an accurate starting point (as opposed to a new node where you have a known starting state).
You could maintain the entire graph's typestate and make the graph generic over it, but then everything depending on the graph becomes volatile. Each operation would consume the graph and create a new one with the updated typestate. This makes it impossible to share across the application (at least as per my knowledge). You could perform actions on the graph locally (i.e. all inline in a sequence), but you wouldn't be able to share it via leptos context, for example.
The other route I explored was to try to store each node's typestate in the node itself (i.e. in the GSOConcrete struct). As far as I can tell this is a non-starter because you can't extricate that type back out. You'd have to type-erase in order to store all of the nodes in a storage structure (like a hashmap), and then that kind of defeats the purpose.
I'm not actually totally sure what Rust would need to change in order to support the behavior I'm looking for. Essentially, I want to be able to have some PhantomData generic type parameter, and then make it so that changing that generic type doesn't actually change the containing type. In other words, I want the typestate to be able to change while maintaining a reference to the underlying data (rather than consuming the entire construct and building it anew every time the typestate changes).
In any case, it seems like the remaining option is to accept some level of runtime checking. Kind of a bummer, from this perspective.
Ideally, we could keep the compile-time checks for building new nodes, and then runtime-check edit operations.
That might complicate the codebase a bit.

## October 27, 2024
Entering a new phase of thinking. Starting to see the system as a kind of language pipeline.
Starting to work on a new schema-editor, and using the system to define an interface into itself feels like the first recursive step.
You could imagine a system where you could change the very schema primitives by making edits to some underlying "metaschema schema".
It seems that in order to manage something like this you'd need some truly primitive layer (probably just consisting of the concepts of nodes and edges).
This layer could be used to build higher-level abstractions which are ultimately just views on patterns of primitives.
It gets quite difficult to figure out how to think about where or how the act of schema-editing comes in, and how it would be propagated through the layers of abstraction to create observable changes in the schema metastructure. Maybe you'd need versions? It seems like with those version artifacts and a base-layer compiler, you could autogenerate abstraction layers which allow interfacing with that particular metaschema-version artifact.
Not loving the idea of versioning, but a better solution is elusive.
As nice as it would be to have the current idea of Templates, Operatives, Instances, and Traits be the universal primitives, it seems like there would be value in finding some truly basic universal primitives like node and directed edge. Probably be worth it in the long run but the details of how all of these ideas will work together are not yet clear.

## November 6, 2024
Hypothesis: Schemas ought to be able to be based on one another or reference one another.
Perhaps the priority at the moment is figuring out how to interpret the *output* of a managed graph environment as a schema for another environment.
In other words, if I have a graph environment which exposes the primitives for schema-building (i.e. templates, operatives, traits), and I create a new schema in that environment (e.g. for rich text editing), how do I use the artifact from the original environment as the basis/schema for building the second environment?

## January 23, 2025
In the process of trying to build out a basic execution-graph model for the purpose of :
1. allowing operatives to have "methods" which traverse their inner graph structure and return some value.
2. allowing those methods to accumulate mutations to the graph to be performed in one atomic mutation after the execution graph has completed.

This is proving somewhat complex for me, as expected, and I'm learning much as I go. In order to have adequate expressiveness to express some target functionalities, it ends up looking a lot like a very basic programming language in which you write directly to a sort of Abstract Syntax Tree (AST) graph.
This has some interesting implications, and it will be both challenging and hopefully enlightening to flesh it out more.
For example, a target functionality that is being used as a guide: a method like `GetLastWordInSentence` on a Sentence operative, where a Sentence has potentially unlimited elements which are either words or punctuations, and each element has a `Next` slot which has zero or one elements.

There have been some choices made for convenience which hopefully can ultimately be remedied or improved upon in future iterations of the system.
As it is, the primitives being used to describe the AST (and other new features) are not very expressive. The next iteration should have things such as slot specialization and (hopefully) the basic ability to reference and find constituent parts within its structure.
For now, I've opted to settle for an untyped interface to the execution graph, where type-erased "ImplData" nodes represent the data flowing through the execution, and the execution-creator is responsible for ensuring that the correct types of data are being used.

Starting with just a pretty bare set of "steps" (which act as data-transformers):
- Bool operations (and, or, not)
- Int comparison operations (equals, less than, greater than)
- If
- MultiTypeSplitter (like a match statement for MultiOperatives)
- Traverse Slot (step into an operative's slot which could yield some range (0, 1, or more) of one of the following: SingleOperative, MultiOperative, TraitOperative)
- IteratorMap (take a collection of things and return a collection of a different type based on a user-provided closure)
- IteratorFilter (take a collection of things and filter out ones which return false in a user-provided closure)
- While Loop
- Mutate Field
- Mutate Slot

For the sake of documentation, it might be a good idea to write down the idea behind the current structure of the `If` step (even if it might be embarrassingly complex at some future date). It is the most complex Step right now.
The `If` construct has the following slots:
- Condition: connects normally to the execution-graph-at-large, must be a bool.
- TrueCaptures and FalseCaptures: Specifies data from the wider graph which will be needed inside of the True and False branches. Can be thought of as capturing data from the graph at large like a closure and marking them to be placed at the start of their respective branch inside the `If`'s TrueBranches and FalseBranches.
- TrueBranches and FalseBranches: The starting points of the true and false branch. There will be one branch per capture. All TrueBranches and FalseBranches must eventually converge (see Convergence slot).
- Convergence: Some data type which both the true and false branch must eventually converge to, thus specifying the return value from each branch. The idea is to hide the "magic" required to choose the branch and let the output node behave more standardly.
- Output: How the rest of the graph interacts with the final output of the `If` construct -- operates the same as outputs from any other Step.

Soundness Invariant: The user is required to ensure that scoping is adhered to; multiple `If`s cannot mingle their branches or use each others' convergence nodes.
On a related note: Been exploring ideas regarding the "conditionality" at conditional split points (`if` and `MultiTypeSplitter`). Common convention is to force convergence of all branches into single type and node, which is what this version will do. Interested in future versions in exploring the possibility of maintaining the conditionality state and allowing branches to interact outside of their conditionality scope.
It could open some interesting possibilities with regards to explicitly handling possibility-spaces as first-class concepts rather than using things like mutable external variables to keep track of information regarding which execution paths ended up running.

Conceptually, the aim is to facilitate a kind of lazy evaluation or "polling" mechanism, where, starting from the output of a function, the data nodes are followed upstream, computing things as necessary.
This is still in the schema-design phase, though, and the implementation of the code generation is as-of-now not started, and that is where these polling ideas will come in. Trying not to block the possibility of this in the schema phase, though.

One concern right now is the interface for *building* these execution graphs (or in other words, for defining a method). The graph-like nature of the control-flows seems to call for some kind of more free-form graph visualization for constructing them.
And even if such an interface is developed, still it's going to be cumbersome and verbose to define methods with much complexity, though hopefully there will be a class of useful and straightforward methods which are enabled.
The hope is that this first version will facilitate more ergonomic and helpful versions in the future.
