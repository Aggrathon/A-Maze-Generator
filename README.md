# Game Maze Generator

TODO: Write more here  
Generate interesting mazes.

## A Quick Introduction to Maze Algorithms

A "perfect" maze is one without loops that has no inaccessible areas. This is the
target of many maze generation algorithms. Another quality of maze generation
algorithms is avoiding patterns, taking this to the extreme is uniform algorithms
where all possible mazes are equally likely to be generated.

A More detailed overview of different maze designs and algorithms can be found here:
[http://www.astrolog.org/labyrnth/algrithm.htm](http://www.astrolog.org/labyrnth/algrithm.htm)

## Algorithm Design Goals

When designing for a situation without global knowledge about the maze, like first-person
view in real life or in a video game these mathematical properties might not be the
best measures of a good maze. They, of course, contribute to the quality of a maze,
but some additions can improve the fun-value. My proposed additions are:

 1. Structures: Non-standard features that can have unique looks or
    functionality, eg. a big tree or a bridge.
 1. Possible loops: Make the left-turn rule infeasible
 1. Remove useless decisions: Help people focus on things that matter.

## Algorithm Structure

TODO: Write More here
The implementation is based on a four step process:

1. Generate/Define Structures:  
    - Start by creating the maze area and any static features, eg. start,
        end, and structures.
    - Use Wilson's algorithm starting from all the doors (or equivalent
        openings). This is done to ensure that no door have a wall on
        either side.
    - Having structures with multiple "doors" are what may create loops
    - Structures are optional, but if not used then this algorithm
        reduces to the Wilson's algorithm.
1. Fill in the rest of the maze with Wilson's algorithm.
    - So most of the maze will share the properties of Wilson's
    algorithm (perfection and uniformity).
1. Connect all separate parts of the maze with Kruskal's algorithm.
    - To make sure that there are no inaccessible areas.
1. Remove all dead-ends of length 1
    - At an intersection, if one branch is only one tile long,
        then remove it.
    - This is an optional step.

### Wilson's algorithm
TODO: Write

### Kruskal's algorithm
TODO: Write
