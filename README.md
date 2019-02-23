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
    functionality, a big tree, a house, a fountain, etc.
 1. Possible loops: Make the left-turn rule infeasible
 1. Remove useless decisions: Help people focus on things that matter.

## Algorithm Structure

The algorithm is essentially a combination of two existing maze algorithms,
Wilson's and Kruskal's (short descriptions below), with the ideas outline
above. The implementation is based on a five step process:

1. Generate/Define Structures (Optional)
    - Start by creating the maze area and any static features, eg. start,
        end, and structures.
1. Ensure no blocked doors (Optional)
    - Start Wilson's algorithm from all doors
    - Having structures with multiple "doors" is what may create loops
    - Loops can (optionally) be avoided by cutting the (Wilson) paths
        that leads to loops in half
1. Fill in the rest of the maze with Wilson's algorithm.
    - So most of the maze will share the properties of Wilson's
        algorithm (perfection and uniformity).
1. Connect all separate parts of the maze with Kruskal's algorithm.
    - To make sure that there are no inaccessible areas.
    - If no structures, start or end is defined, then this step is
        not used (the algorithm reduces to Wilson's algorithm)
1. Remove all dead-ends of length 1 (Optional)
    - At an intersection, if one branch is only one tile long,
        then remove it.

### Wilson's algorithm

From each unadded node in the graph do a random walk until
you hit an existing part of the maze. Then backtrack the walk
while removing loops and add the nodes to the maze.

The random walk makes the time required indeterministic, which worse with
larger mazes. In this implementation I added the constraint that
the random walk cannot cross itself. This requires the introduction of
backtracking when the algorithm has "painted itself into a corner".
The results is a strict upper bound on the time required (O(n) where n
is the size of the maze).

### Kruskal's algorithm

Start with each node in its own set. For each pair of adjacent nodes
if they are not in the same set create a passage between them and merge
their sets.

## Implementation

In this repo contains a simple implementation in Rust. This implementation
only works on a 2D grid with simple structures (no bridges, teleports, etc.),
but these limitations aren't inherent to the algorithm.

Included is also maze visualisations in both text and image formats. As a
bonus there is a random recursive backtracking solver which can be used to
draw different paths between the start and the end. For examples, see below.

More mazes can be generated with: `cargo run`

## Examples

> TODO: Insert images
