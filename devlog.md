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
