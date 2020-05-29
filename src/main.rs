struct Board {
	pub inner: [[Option<u8>; 9]; 9]
}
struct PossibleSet {
	inner: u16
}
impl PossibleSet {
	pub fn size(&self) -> usize {
		let mut size = 0;
		for offset in 0..9 {
			if self.inner & 1 << offset != 0 {
				size += 1;
			}
		}

		size
	}
	pub fn remove(&mut self, other: Self) {
		self.inner &= !other.inner;
	}
	pub fn all() -> Self {
		Self {
			inner: 0b111111111
		}
	}
}
impl From<u8> for PossibleSet {
	fn from(num: u8) -> Self {
		Self {
			inner: match num {
				1 => 1 << 0,
				2 => 1 << 1,
				3 => 1 << 2,
				4 => 1 << 3,
				5 => 1 << 4,
				6 => 1 << 5,
				7 => 1 << 6,
				8 => 1 << 7,
				9 => 1 << 8,
				_ => unreachable!()
			}
		}
	}
}
impl From<PossibleSet> for u8 {
	fn from(set: PossibleSet) -> Self {
		if set.size() == 1 {
			if set.inner == 1 << 0 {
				1
			} else if set.inner == 1 << 1 {
				2
			} else if set.inner == 1 << 2 {
				3
			} else if set.inner == 1 << 3 {
				4
			} else if set.inner == 1 << 4 {
				5
			} else if set.inner == 1 << 5 {
				6
			} else if set.inner == 1 << 6 {
				7
			} else if set.inner == 1 << 7 {
				8
			} else if set.inner == 1 << 8 {
				9
			} else {
				unreachable!()
			}
		} else {
			unreachable!()
		}
	}
}
impl Board {
	// const TEST1: Board = Board {
	// 	inner: [
	// 		[None   , Some(1), None   ,  Some(4), Some(9), None   ,  Some(7), None   , None   ],
	// 		[None   , None   , None   ,  None   , Some(7), None   ,  Some(8), None   , Some(6)],
	// 		[Some(2), Some(3), None   ,  None   , None   , None   ,  None   , None   , None   ],

	// 		[None   , None   , None   ,  None   , None   , None   ,  None   , None   , Some(5)],
	// 		[Some(6), Some(9), None   ,  None   , Some(5), None   ,  None   , Some(1), Some(7)],
	// 		[Some(7), None   , None   ,  None   , None   , None   ,  None   , None   , None   ],
			
	// 		[None   , None   , None   ,  None   , None   , None   ,  None   , Some(9), Some(8)],
	// 		[Some(4), None   , Some(5),  None   , Some(2), None   ,  None   , None   , None   ],
	// 		[None   , None   , Some(1),  None   , Some(8), Some(4),  None   , Some(7), None   ]
	// 	]
	// };

	fn neighbors(row: usize, column: usize) -> Vec<(usize, usize)> { // Vec length will always be twenty
		let mut list = Vec::new();

		// Add Square:
		let rb = row - row % 3;
		let cb = column - column % 3;
		for ri in rb..(rb + 3) {
			for ci in cb..(cb + 3) {
				if ri != row || ci != column {
					list.push((ri, ci));
				}
			}
		}

		// Add Row:
		match row {
			0 | 1 | 2 => list.extend_from_slice(&[(3, column), (4, column), (5, column), (6, column), (7, column), (8, column)]),
			3 | 4 | 5 => list.extend_from_slice(&[(0, column), (1, column), (2, column), (6, column), (7, column), (8, column)]),
			6 | 7 | 8 => list.extend_from_slice(&[(0, column), (1, column), (2, column), (3, column), (4, column), (5, column)]),
			_ => unreachable!()
		}

		// Add Column:
		match column {
			0 | 1 | 2 => list.extend_from_slice(&[(row, 3), (row, 4), (row, 5), (row, 6), (row, 7), (row, 8)]),
			3 | 4 | 5 => list.extend_from_slice(&[(row, 0), (row, 1), (row, 2), (row, 6), (row, 7), (row, 8)]),
			6 | 7 | 8 => list.extend_from_slice(&[(row, 0), (row, 1), (row, 2), (row, 3), (row, 4), (row, 5)]),
			_ => unreachable!()
		}

		list
	}

	// TODO: Add seed
	pub fn generate() -> Self {
		// Generate a full board that meets sudoku criteria:
		let mut board = Board {
			inner: [[None; 9]; 9]
		};
		if !board.solve() {
			unreachable!()
		} // TODO: Make new function - fill instead.
		board.print();

		// Remove cells untill we no longer can:
		loop {
			let mut made_harder = false;

			for ri in 0..9 {
				for ci in 0..9 {
					if let Some(num) = board.inner[ri][ci] {
						// Check if we can remove this cell because it's neighbors have all the other numbers:
						let mut possibles = PossibleSet::all();
						for (rii, cii) in Self::neighbors(ri, ci) {
							if let Some(num) = board.inner[rii][cii] {
								possibles.remove(num.into());
							}
						}
						if possibles.size() == 1 {
							board.inner[ri][ci] = None;
							made_harder = true;
						}
					}
				}
			}

			if !made_harder {
				break;
			}
		}

		board
	}
	pub fn verify(&self, row: usize, column: usize) -> bool {
		let test = self.inner[row][column];
		if let None = test {
			return true;
		}

		// Check Neighbors:
		for (ri, ci) in Self::neighbors(row, column) {
			let item = self.inner[ri][ci];
			if item == test {
				return false;
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
	let mut board = Board::generate();
	board.print();

	println!("Solved? {}", board.solve());
	board.print();
	
	// for (r, c, cell) in board.board.iter().enumerate().flat_map(|(r, row)| row.iter().enumerate().map(move |(c, cell)| (r, c, cell))) {
	// 	println!("Row: {} Column: {} Cell: {:?}", r, c, cell);
	// }
}
