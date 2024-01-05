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

## Workflow

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
