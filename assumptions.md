# Assumptions

## Terms:
Ancestry: Refers to the possible list of decreasingly specific operatives which an operative might be based on, ultimately ending at the template.
Constituent Hierarchy: Refers to the tree of operatives which form the structure of a template.

## Assumptions that are not enforced
1. Templates cannot contain anywhere in their constituent hierarchy an operative based on the same template.
2. Traits should be implemented only once in an operative ancestry
<!-- 3. Generic refinement must terminate in an ancestry:  -->
<!--   a. if/where an operative is supplied. -->
<!--   b. if/where an instance is slotted. -->
<!--   In other words, new trait constraints can be added to the generic up until a specific operative is supplied or an instance is slotted. -->
