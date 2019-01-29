
use super::Maze;

pub fn set_join(maze: &mut Maze, index: usize) {
    let set: i32 = vec![index, index - 1, index + 1, index - maze.width, index + maze.width]
        .iter().fold(maze.counter, |x, di| if maze.maze[*di] > 0 && maze.maze[*di] < x { maze.maze[*di] } else { x });
    if set == maze.counter { maze.counter += 1; }
    let mut stack = vec![index];
    maze.maze[index] = set;
    while !stack.is_empty() {
        let i = stack.pop().unwrap();
        let x = i%maze.width;
        let y = i/maze.width;
        if x+1 < maze.width && maze.maze[i+1] > set { stack.push(i+1); maze.maze[i+1] = set; }
        if x > 0 && maze.maze[i-1] > set { stack.push(i-1); maze.maze[i-1] = set; }
        if y+1 < maze.height && maze.maze[i+maze.width] > set { stack.push(i+maze.width); maze.maze[i+maze.width] = set; }
        if y > 0 && maze.maze[i-maze.width] > set { stack.push(i-maze.width); maze.maze[i-maze.width] = set; }
    }
}