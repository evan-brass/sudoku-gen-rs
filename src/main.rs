#![allow(dead_code)]



struct Board {
	pub inner: [[Option<u8>; 9]; 9]
}
impl Board {
	const N: Option<u8> = None;
	const A: Option<u8> = Some(1);
	const B: Option<u8> = Some(2);
	const C: Option<u8> = Some(3);
	const D: Option<u8> = Some(4);
	const E: Option<u8> = Some(5);
	const F: Option<u8> = Some(6);
	const G: Option<u8> = Some(7);
	const H: Option<u8> = Some(8);
	const I: Option<u8> = Some(9);
	const TEST1: Board = Board {
		inner: [
			[Self::N, Self::A, Self::N,  Self::D, Self::I, Self::N,  Self::G, Self::N, Self::N],
			[Self::N, Self::N, Self::N,  Self::N, Self::G, Self::N,  Self::H, Self::N, Self::F],
			[Self::B, Self::C, Self::N,  Self::N, Self::N, Self::N,  Self::N, Self::N, Self::N],

			[Self::N, Self::N, Self::N,  Self::N, Self::N, Self::N,  Self::N, Self::N, Self::E],
			[Self::F, Self::I, Self::N,  Self::N, Self::E, Self::N,  Self::N, Self::A, Self::G],
			[Self::G, Self::N, Self::N,  Self::N, Self::N, Self::N,  Self::N, Self::N, Self::N],
			
			[Self::N, Self::N, Self::N,  Self::N, Self::N, Self::N,  Self::N, Self::I, Self::H],
			[Self::D, Self::N, Self::E,  Self::N, Self::B, Self::N,  Self::N, Self::N, Self::N],
			[Self::N, Self::N, Self::A,  Self::N, Self::H, Self::D,  Self::N, Self::G, Self::N]
		]
	};
	pub fn verify(&self, row: usize, column: usize) -> bool {
		let test = self.inner[row][column];
		if let None = test {
			return true;
		}

		// Check row
		for (ci, item) in self.inner[row].iter().enumerate() {
			if ci != column && *item == test {
				return false;
			}
		}

		// Check column
		for ri in 0..9 {
			let item = self.inner[ri][column];
			if ri != row && item == test {
				return false;
			}
		}

		// check square
		let rb = row - row % 3;
		let cb = column - column % 3;
		for ri in rb..(rb + 3) {
			for ci in cb..(cb + 3) {
				let item = self.inner[ri][ci];
				if (ri != row || ci != column) && item == test {
					return false;
				}
			}
		}

		true
	}
	pub fn solve(&mut self) -> bool {
		self.solve_sub(0, 0)
	}
	fn solve_sub(&mut self, row: usize, column: usize) -> bool {
		fn next_indexes(row: usize, column: usize) -> Option<(usize, usize)> {
			if column == 8 {
				if row == 8 {
					None
				} else {
					Some((row + 1, 0))
				}
			} else {
				Some((row, column + 1))
			}
		}
		if let None = self.inner[row][column] {
			// TODO: Shuffle parts
			let mut parts: [u8; 9] = [1,2,3,4,5,6,7,8,9];
			for test in &parts {
				self.inner[row][column] = Some(*test);
				if self.verify(row, column) {
					if let Some((r, c)) = next_indexes(row, column) {
						if self.solve_sub(r, c) {
							return true;
						}
					} else {
						return true;
					}
				}
			}
			self.inner[row][column] = None;
			false
		} else {
			// This is an initial square - skip:
			if let Some((r, c)) = next_indexes(row, column) {
				self.solve_sub(r, c)
			} else {
				// We've reached the end! Successfully solved.
				true
			}
		}
	}

	pub fn print(&self) {
		// 
		print!("╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n");
		for (ri, row) in self.inner.iter().enumerate() {
			print!("║");
			for (ci, item) in row.iter().enumerate() {
				print!(" ");
				match item {
					Some(num) => print!("{}", num),
					None => print!(" ")
				}
				if (ci + 1) % 3 == 0 {
					print!(" ║");
				} else {
					print!(" │");
				}
			}
			if (ri + 1) % 9 == 0 {
				print!("\n╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝\n");
			} else if (ri + 1) % 3 == 0 {
				print!("\n╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n");
			} else {
				print!("\n╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n");
			}
		}
	}
}

fn main() {
	let mut board = Board::TEST1;
	// let mut board = Board {
	// 	inner: [[None; 9]; 9]
	// };
	board.print();

	println!("Solved? {}", board.solve());
	board.print();
	
	// for (r, c, cell) in board.board.iter().enumerate().flat_map(|(r, row)| row.iter().enumerate().map(move |(c, cell)| (r, c, cell))) {
	// 	println!("Row: {} Column: {} Cell: {:?}", r, c, cell);
	// }
}
