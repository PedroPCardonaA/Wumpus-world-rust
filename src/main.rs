use std::{cell, collections::{HashMap, HashSet}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CellAttribute {
    Wumpus,
    Pit,
    Gold,
    Stench,
    Breeze,
    Glitter,
}

#[derive(Debug)]
struct World {
    grid: Vec<Vec<HashSet<CellAttribute>>>,
    agent_position: (usize, usize),
    agent_direction: usize,
    arrow: bool,
}

impl World{
    fn new(size: usize) -> World{
        let grid = vec![vec![HashSet::new(); size]; size];
        World{
            grid,
            agent_position: (0, 0),
            agent_direction: 90,
            arrow: true,
        }
    }

    fn place_wumpus(&mut self){
        let size = self.grid.len();
        let x = rand::random::<usize>() % (size - 1) + 1;
        let y = rand::random::<usize>() % (size - 1) + 1;
        self.grid[x][y].insert(CellAttribute::Wumpus);
        self.add_stench(x, y);
    }

    fn add_stench(&mut self, x: usize, y: usize){
        if x > 0 {
            self.grid[x - 1][y].insert(CellAttribute::Stench);
        }
        if x < self.grid.len() - 1 {
            self.grid[x + 1][y].insert(CellAttribute::Stench);
        }
        if y > 0 {
            self.grid[x][y - 1].insert(CellAttribute::Stench);
        }
        if y < self.grid.len() - 1 {
            self.grid[x][y + 1].insert(CellAttribute::Stench);
        }
    }

    fn place_pits(&mut self){
        let size = self.grid.len();
        let mut pits = size/ 4 * 3;
        while pits > 0 {
            let x = rand::random::<usize>() % size;
            let y = rand::random::<usize>() % size;
            if self.grid[x][y].is_empty() {
                self.grid[x][y].insert(CellAttribute::Pit);
                self.add_breeze(x, y);
                pits -= 1;
            }
        }
    }

    fn add_breeze(&mut self, x: usize, y: usize){
        if x > 0 {
            self.grid[x - 1][y].insert(CellAttribute::Breeze);
        }
        if x < self.grid.len() - 1 {
            self.grid[x + 1][y].insert(CellAttribute::Breeze);
        }
        if y > 0 {
            self.grid[x][y - 1].insert(CellAttribute::Breeze);
        }
        if y < self.grid.len() - 1 {
            self.grid[x][y + 1].insert(CellAttribute::Breeze);
        }
    }

    fn place_gold_and_glitter(&mut self){
        let size = self.grid.len();
        let x = rand::random::<usize>() % (size - 1) + 1;
        let y = rand::random::<usize>() % (size - 1) + 1;
        self.grid[x][y].insert(CellAttribute::Gold);
        self.grid[x][y].insert(CellAttribute::Glitter);
    }

    fn percept(&self) -> HashSet<CellAttribute>{
        let (x, y) = self.agent_position;
        self.grid[x][y].clone()
    }

    fn rotate_agent(&mut self, left: bool){
        if left {
            self.agent_direction = (self.agent_direction + 270) % 360;
        } else {
            self.agent_direction = (self.agent_direction + 90) % 360;
        }
    }
    
    fn move_agent(&mut self){
        let (x, y) = self.agent_position;
        let mut new_x = x;
        let mut new_y = y;
        match self.agent_direction {
            0 => new_y += 1,
            90 => new_x += 1,
            180 => new_y -= 1,
            270 => new_x -= 1,
            _ => panic!("Invalid direction"),
        }
        if new_x < self.grid.len() && new_y < self.grid.len() {
            self.agent_position = (new_x, new_y);
        }
    }

    fn shot_arrow(&mut self) {
        if self.arrow {
            self.arrow = false; 
            let (mut x, mut y) = self.agent_position;
    
            loop {
                match self.agent_direction {
                    0 => y += 1,   
                    90 => x += 1,  
                    180 => if y > 0 { y -= 1 }, 
                    270 => if x > 0 { x -= 1 },
                    _ => panic!("Invalid direction"),
                }
    
                if x >= self.grid.len() || y >= self.grid.len() {
                    println!("The arrow hit a wall and is lost.");
                    break;
                }
                if self.grid[x][y].contains(&CellAttribute::Wumpus) {
                    println!("The arrow hit and killed the Wumpus!");
                    self.grid[x][y].remove(&CellAttribute::Wumpus);
                    break;
                }
            }
        } else {
            println!("No arrows left to shoot!");
        }
    }

}

struct Agent{
    knowledge_base: HashMap<(usize, usize), HashSet<CellAttribute>>,
    safe_cells: HashSet<(usize, usize)>,
    wumpus_killed: bool,
}

impl Agent{
    fn new() -> Agent{
        let mut agent = Agent{
            knowledge_base: HashMap::new(),
            safe_cells: HashSet::new(),
            wumpus_killed: false,
        };
        agent.safe_cells.insert((0, 0));
        agent
    }

    fn update_knowledge(&mut self, pos: (usize, usize), percepts: HashSet<CellAttribute>){
        self.knowledge_base.insert(pos,percepts.clone());
        if !percepts.contains(&CellAttribute::Breeze) && !percepts.contains(&CellAttribute::Stench){
            self.safe_cells.insert(pos);
        }
        if percepts.contains(&CellAttribute::Glitter){
            println!("I found the gold!");
        }
    }

    fn decide_next_move(&self, world: &World) -> Option<(usize, usize)>{
        for safe_cell in &self.safe_cells {
            if !self.knowledge_base.contains_key(safe_cell) {
                return Some(*safe_cell);
            }
        }
        None
    }

}

fn main() {
    let mut world = World::new(4);
    world.place_wumpus();
    world.place_pits();
    world.place_gold_and_glitter();

    let mut agent = Agent::new();

    let percepts = world.percept();
    agent.update_knowledge(world.agent_position, percepts.clone());

    loop {
        if let Some(next_move) = agent.decide_next_move(&world) {
            println!("Agent moves to {:?}", next_move);
            world.agent_position = next_move;

            let percepts = world.percept();
            agent.update_knowledge(next_move, percepts.clone());

            if percepts.contains(&CellAttribute::Glitter) {
                break;
            }
        } else {
            println!("No safe move available, agent is stuck!");
            break;
        }
    }
}





