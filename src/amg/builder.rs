
use image::Rgb;
use super::Maze;

pub struct MazeBuilder {
    height: usize,
    width: usize,
    _image: bool,
    _solve: bool,
    _exit: bool,
    _loops: bool,
    _stubs: bool,
    _structs: bool,
    _name: String
}

impl Maze {
    pub fn builder(width: usize, height: usize) -> MazeBuilder {
        MazeBuilder{
            width, height,
            _image: false,
            _solve: false,
            _exit: true,
            _loops: true,
            _stubs: false,
            _structs: true,
            _name: String::from("Maze")
        }
    }
}

impl MazeBuilder {
    pub fn image(&mut self, val: bool) -> &mut MazeBuilder {
        self._image = val;
        self
    }
    pub fn solve(&mut self, val: bool) -> &mut MazeBuilder {
        self._solve = val;
        self
    }
    pub fn exits(&mut self, val: bool) -> &mut MazeBuilder {
        self._exit = val;
        self
    }
    pub fn loops(&mut self, val: bool) -> &mut MazeBuilder {
        self._loops = val;
        self
    }
    pub fn stubs(&mut self, val: bool) -> &mut MazeBuilder {
        self._stubs = val;
        self
    }
    pub fn structures(&mut self, val: bool) -> &mut MazeBuilder {
        self._structs = val;
        self
    }
    pub fn filename(&mut self, val: String) -> &mut MazeBuilder {
        self._name = val;
        self
    }
    pub fn parse_word(&mut self, word: &str) -> &mut MazeBuilder {
        match word.as_ref() {
            "image" => self.image(true),
            "solve" => self.solve(true),
            "no-exit" => self.exits(false),
            "no-loops" => self.loops(false),
            "no-stubs" => self.stubs(false),
            "no-struct" => self.structures(false),
            &_ => self
        }
    }
    pub fn parse_letter(&mut self, letter: char) -> &mut MazeBuilder {
        match letter {
            'i' => self.image(true),
            'o' => self.solve(true),
            'e' => self.exits(false),
            'l' => self.loops(false),
            't' => self.stubs(false),
            's' => self.structures(false),
            _ => self
        }
    }
    pub fn build(&self) -> Maze {
        let mut maze = Maze::new(self.width, self.height, self._exit);
        if self._structs {
            maze.generate_structures_default();
        }
        maze.generate(self._loops);
        if self._stubs {
            maze.remove_stubs();
        }
        if self._image {
            maze.to_image(3).save(self._name.to_string()+".png").unwrap();
        }
        if self._solve {
            let mut image = maze.to_image_color(6);
            for (i, c) in [
                Rgb([255, 128, 128]),
                Rgb([128, 128, 255]),
                Rgb([255, 64, 255]),
                Rgb([64, 255, 255]),
                Rgb([64, 255, 128]),
                Rgb([255, 255, 128])
            ].iter().enumerate() {
                let path = super::solve::recursive_backtracker(&maze, maze.width/2, maze.maze.len()-maze.width/2-1);
                super::image::add_path_to_maze_image(&maze, &path, &mut image, *c, i as u32);
            }
            image.save(self._name.to_string()+"_solved.png").unwrap();
        }
        maze.print();
        maze
    }
}