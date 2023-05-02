use rand::Rng;

#[derive(Copy, Clone)]
pub enum Direction {
    Right, Down,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn index(&self, maze_width: usize) -> usize {
        self.y as usize * maze_width + self.x as usize
    }
}

#[derive(Copy, Clone)]
pub struct Connection {
    start: Point,
    dir: Direction,
    weight: u32,
    include: bool,
    in_solution: bool,
}

impl Connection {
    pub fn get_start(&self) -> Point {
        self.start
    }

    pub fn get_end(&self) -> Point {
        match self.dir {
            Direction::Right => return Point {
                x: self.start.x + 1,
                y: self.start.y,
            },
            Direction::Down => return Point {
                x: self.start.x,
                y: self.start.y + 1,
            },
        }
    }

    pub fn get_dir(&self) -> Direction {
        self.dir
    }

    pub fn is_included(&self) -> bool {
        self.include
    }

    pub fn is_in_solution(&self) -> bool {
        self.in_solution
    }
}

pub struct Maze {
    points: Vec<Vec<Point>>,
    connections: Vec<Connection>,
    start: Point,
    end: Point,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut rng = rand::thread_rng();

        let mut points = vec![vec![Point {x:0,y:0,}; height]; width];
        for i in 0..width {
            for j in 0..height {
                points[i][j].x = i as u32;
                points[i][j].y = j as u32;
            }
        }

        let mut connections = Vec::new();
        for i in 0..width-1 {
            for j in 0..height {
                connections.push(Connection {
                    start: points[i][j],
                    dir: Direction::Right,
                    weight: rng.gen_range(0..10),
                    include: false, in_solution: false,
                });
            }
        }

        for i in 0..width {
            for j in 0..height-1 {
                connections.push(Connection {
                    start: points[i][j],
                    dir: Direction::Down,
                    weight: rng.gen_range(0..10),
                    include: false, in_solution: false,
                });
            }
        }

        connections.sort_by_key(|connection| connection.weight);

        let start = Point {
            x: 0,
            y: rng.gen_range(0..height as u32),
        };
        let end = Point {
            x: width as u32 - 1,
            y: rng.gen_range(0..height as u32),
        };

        Maze {
            points, connections, start, end,
        }
    }

    pub fn get_width(&self) -> usize {
        self.points.len()
    }

    pub fn get_height(&self) -> usize {
        self.points[0].len()
    }

    pub fn get_connection(&self, i: usize) -> Connection {
        self.connections[i]
    }

    pub fn connection_by_points(&self, p1: Point, p2: Point) -> Option<usize> {
        self.connections.iter().position(
            |&c| ((c.start.x == p1.x && c.start.y == p1.y)
            && (c.get_end().x == p2.x && c.get_end().y == p2.y))
            || ((c.start.x == p2.x && c.start.y == p2.y)
            && (c.get_end().x == p1.x && c.get_end().y == p1.y))
        )
    }

    pub fn num_connections(&self) -> usize {
        self.connections.len()
    }

    pub fn include_connection(&mut self, i: usize) {
        self.connections[i].include = true;
    }

    pub fn add_to_solution(&mut self, i: usize) {
        self.connections[i].in_solution = true;
    }

    pub fn get_start(&self) -> Point {
        self.start
    }

    pub fn get_end(&self) -> Point {
        self.end
    }
}