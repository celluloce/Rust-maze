#[derive(PartialEq)]
enum MazeKind {
	// ブロックの種類
	Path,
	Wall,
	Start,
	Goal,
}

struct TypeDef {
	kind: MazeKind,
	flag: bool,
}

impl TypeDef {
	fn new_path(flag: bool) -> Self {
		TypeDef {
			kind: MazeKind::Path,
			flag: flag,
		}
	}
	fn new_wall(flag: bool) -> Self {
		TypeDef {
			kind: MazeKind::Wall,
			flag: flag,
		}
	}
	fn new_start(flag: bool) -> Self {
		TypeDef {
			kind: MazeKind::Start,
			flag: flag,
		}
	}
	fn new_goal(flag: bool) -> Self {
		TypeDef {
			kind: MazeKind::Goal,
			flag: flag,
		}
	}
}

#[derive(Debug)]
struct Player {
	x: usize,
	y: usize,
	goal: bool,
}

impl Player {
	fn new() -> Self {
		Player {
			x: 0,
			y: 0,
			goal: false,
		}
	}
}

const MAZE_ROW: usize = 5;
const MAZE_COLUMN: usize = 5;

fn main() {
	let mut maze: [[TypeDef; MAZE_ROW]; MAZE_COLUMN] = [
		[TypeDef::new_start(true), TypeDef::new_path(false), TypeDef::new_path(false), TypeDef::new_path(false), TypeDef::new_path(false) ],
		[TypeDef::new_wall(false), TypeDef::new_wall(false), TypeDef::new_path(false), TypeDef::new_wall(false), TypeDef::new_wall(false) ],
		[TypeDef::new_wall(false), TypeDef::new_path(false), TypeDef::new_path(false), TypeDef::new_path(false), TypeDef::new_path(false) ],
		[TypeDef::new_path(false), TypeDef::new_path(false), TypeDef::new_wall(false), TypeDef::new_wall(false), TypeDef::new_wall(false) ],
		[TypeDef::new_wall(false), TypeDef::new_path(false), TypeDef::new_path(false), TypeDef::new_path(false), TypeDef::new_goal(true) ]
	];

	let mut player = Player::new();
	maze_drow(&maze, &player);

	loop {
		use std::io::*;
		println!("----------");
		println!("キーを押してEnter");
		println!("w: 上, s: 下, d: 右, a: 左\n");

		let mut input_s = String::new();
		stdin().read_line(&mut input_s).expect("Failed read_line");

		match input_s.as_str().trim() {
			"w" => move_up(&mut maze, &mut player),
			"s" => move_down(&mut maze, &mut player),
			"d" => move_right(&mut maze, &mut player),
			"a" => move_left(&mut maze, &mut player),
			_ => println!("それは有効なキーではない"),
		}

		maze_drow(&maze, &player);

		if player.goal {
			println!("目的地にたどりついたよ、やったね。");
			break;
		}
	}
}

fn maze_drow(maze: &[[TypeDef; MAZE_ROW]; MAZE_COLUMN], player: &Player) {
	println!("");
	for c in 0..MAZE_ROW {
		for d in 0..MAZE_COLUMN {
			if d == player.x && c == player.y {
				print!("P ");
			} else if maze[c][d].flag == false {
				print!("? ");
			} else {
				match maze[c][d].kind {
					MazeKind::Wall => print!("* "),
					MazeKind::Goal => print!("G "),
					_ => print!("  "),
				}
			}
		}
		println!("");
	}
	println!("");
}

	// maze[row (y)][column (x)]

	fn move_up(maze: &mut [[TypeDef; MAZE_ROW]; MAZE_COLUMN], player: &mut Player) {
		let px = player.x;
		let py = player.y;

		if py > 1 {
			let u_kind = &maze[py - 1][px].kind;

			if u_kind == &MazeKind::Wall {
				println!("そこは壁だ");
				maze[py - 1][px].flag = true;
			} else if u_kind == &MazeKind::Path || u_kind == &MazeKind::Start {
				println!("進めた");
				player.y -= 1;
				maze[py - 1][px].flag = true;
			} else if u_kind == &MazeKind::Goal {
				println!("あ、着いた");
				player.y -= 1;
				player.goal = true;
			}
		} else {
			println!("それ以上上には行けない");
		}
	}
	fn move_down(maze: &mut [[TypeDef; MAZE_ROW]; MAZE_COLUMN], player: &mut Player) {
		let px = player.x;
		let py = player.y;

		if py < MAZE_ROW - 1{
			let u_kind = &maze[py + 1][px].kind;

			if u_kind == &MazeKind::Wall {
				println!("そこは壁だ");
				maze[py + 1][px].flag = true;
			} else if u_kind == &MazeKind::Path || u_kind == &MazeKind::Start {
				println!("進めた");
				player.y += 1;
				maze[py + 1][px].flag = true;
			} else if u_kind == &MazeKind::Goal {
				println!("あ、着いた");
				player.y += 1;
				player.goal = true;
			}
		} else {
			println!("それ以上上には行けない");
		}
	}
	fn move_right(maze: &mut [[TypeDef; MAZE_ROW]; MAZE_COLUMN], player: &mut Player) {
		let px = player.x;
		let py = player.y;

		if px < MAZE_COLUMN - 1 {
			let u_kind = &maze[py][px + 1].kind;

			if u_kind == &MazeKind::Wall {
				println!("そこは壁だ");
				maze[py][px + 1].flag = true;
			} else if u_kind == &MazeKind::Path || u_kind == &MazeKind::Start {
				println!("進めた");
				player.x += 1;
				maze[py][px + 1].flag = true;
			} else if u_kind == &MazeKind::Goal {
				println!("あ、着いた");
				player.x += 1;
				player.goal = true;
			}
		} else {
			println!("それ以上上には行けない");
		}
	}
	fn move_left(maze: &mut [[TypeDef; MAZE_ROW]; MAZE_COLUMN], player: &mut Player) {
		let px = player.x;
		let py = player.y;

		if px > 1 {
			let u_kind = &maze[py][px - 1].kind;

			if u_kind == &MazeKind::Wall {
				println!("そこは壁だ");
				maze[py][px - 1].flag = true;
			} else if u_kind == &MazeKind::Path || u_kind == &MazeKind::Start {
				println!("進めた");
				player.x -= 1;
				maze[py][px - 1].flag = true;
			} else if u_kind == &MazeKind::Goal {
				println!("あ、着いた");
				player.x -= 1;
				player.goal = true;
			}
		} else {
			println!("それ以上上には行けない");
		}
	}
