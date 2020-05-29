use rand::prelude::*;

struct Board {
	pub inner: [[Option<u8>; 9]; 9]
}
struct PossibleSet {
	pub inner: u16
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
	pub fn contains(&mut self, other: Self) -> bool {
		self.inner & other.inner != 0
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
#[derive(Debug)]
enum NeighborKind {
	Row,
	Column,
	Square,
	SquareAndRow,
	SquareAndColumn
}
impl Board {
	#[allow(dead_code)]
	const TEST1: Board = Board {
		inner: [
			[None   , Some(1), None   ,  Some(4), Some(9), None   ,  Some(7), None   , None   ],
			[None   , None   , None   ,  None   , Some(7), None   ,  Some(8), None   , Some(6)],
			[Some(2), Some(3), None   ,  None   , None   , None   ,  None   , None   , None   ],

			[None   , None   , None   ,  None   , None   , None   ,  None   , None   , Some(5)],
			[Some(6), Some(9), None   ,  None   , Some(5), None   ,  None   , Some(1), Some(7)],
			[Some(7), None   , None   ,  None   , None   , None   ,  None   , None   , None   ],

			[None   , None   , None   ,  None   , None   , None   ,  None   , Some(9), Some(8)],
			[Some(4), None   , Some(5),  None   , Some(2), None   ,  None   , None   , None   ],
			[None   , None   , Some(1),  None   , Some(8), Some(4),  None   , Some(7), None   ]
		]
	};
	// TODO: Technically the box is unneccessary but the type is a monster...
	pub fn neighbors(row: usize, column: usize) -> Box<dyn Iterator<Item=(usize, usize, NeighborKind)>> {
		let rb = row - row % 3;
		let cb = column - column % 3;

		// Square
		let it_square = (rb..(rb + 3)).flat_map(move |ri| {
			(cb..(cb + 3)).map(move |ci| {
				(
					ri,
					ci,
					if ri == row {
						NeighborKind::SquareAndRow
					} else if ci == column {
						NeighborKind::SquareAndColumn
					} else {
						NeighborKind::Square
					}
				)
			})
		}).filter(move |(ri, ci, _)| !(*ri == row && *ci == column));

		// Row
		let (ca, cb) = match column {
			0 | 1 | 2 => (3, 6),
			3 | 4 | 5 => (0, 6),
			6 | 7 | 8 => (0, 3),
			_ => unreachable!()
		};
		let it_column = (ca..(ca + 3)).map(move |ci| (row, ci, NeighborKind::Row)).chain((cb..(cb + 3)).map(move |ci| (row, ci, NeighborKind::Row)));

		// Column
		let (ra, rb) = match row {
			0 | 1 | 2 => (3, 6),
			3 | 4 | 5 => (0, 6),
			6 | 7 | 8 => (0, 3),
			_ => unreachable!()
		};
		let it_row = (ra..(ra + 3)).map(move |ri| (ri, column, NeighborKind::Column)).chain((rb..(rb + 3)).map(move |ri| (ri, column, NeighborKind::Column)));

		Box::new(it_square.chain(it_row).chain(it_column))
	}

	// TODO: Add seed
	pub fn generate() -> Self {
		// Generate a full board that meets sudoku criteria:
		let mut board = Board {
			inner: [[None; 9]; 9]
		};

		// TODO: Make new function (fill) instead.
		if !board.solve() {
			unreachable!()
		}

		// Remove cells untill we no longer can:
		loop {
			let mut made_harder = false;

			let mut order: Vec<(usize, usize)> = (0..9).flat_map(|ri| (0..9).map(move |ci| (ri, ci))).collect();
			order.shuffle(&mut thread_rng());

			for (ri, ci) in order {
				// board.print();
				if let Some(num) = board.inner[ri][ci] {
					// Check if we can remove this cell because its neighbors have all the other numbers:
					let mut possibles = PossibleSet::all();
					let mut square_only = true;
					let mut row_only = true;
					let mut column_only = true;
					for (rii, cii, neighbor_kind) in Self::neighbors(ri, ci) {
						use NeighborKind::{Column, Row, Square, SquareAndColumn, SquareAndRow};
						if let Some(num) = board.inner[rii][cii] {
							possibles.remove(num.into());
						} else {
							
							// Get the possibles for this neighbor - excluding whatever cell we're currently checking:
							let mut sub_possibles = PossibleSet::all();
							for (riii, ciii, _) in Self::neighbors(rii, cii) {
								if !(riii == ri && ciii == ci) {
									if let Some(sub_num) = board.inner[riii][ciii] {
										sub_possibles.remove(sub_num.into());
									}
								}
							}
							// println!("Checking only's for ({}, {}) at ({}, {}): {:b}", ri, ci, rii, cii, sub_possibles.inner);
							if sub_possibles.contains(num.into()) {
								match neighbor_kind {
									Square => square_only = false,
									SquareAndRow => {
										square_only = false;
										row_only = false;
									},
									SquareAndColumn => {
										square_only = false;
										column_only = false;
									},
									Row => row_only = false,
									Column => column_only = false
								}
							}
						}
					}
					if possibles.size() == 1 || square_only || row_only || column_only {
						board.inner[ri][ci] = None;
						made_harder = true;
						// println!("Removed ({}, {}) - p {:b}, s {} r {} c {}", ri, ci, possibles.inner, square_only, row_only, column_only);
					} else {
						// println!("Couldn't remove ({}, {}) - p {:b}", ri, ci, possibles.inner);
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
		for (ri, ci, _) in Self::neighbors(row, column) {
			let item = self.inner[ri][ci];
			if item == test {
				// println!("Failed verify for ({}, {}) because of ({}, {})", row, column, ri, ci);
				return false;
			}
		}
		true
	}
	pub fn solve(&mut self) -> bool {
		self.solve_sub(0, 0)
	}
	fn solve_sub(&mut self, row: usize, column: usize) -> bool {
		// println!("Solving for ({}, {})", row, column);
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
			parts.shuffle(&mut thread_rng());

			for test in &parts {
				self.inner[row][column] = Some(*test);
				// self.print();
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
	// for neighbor in Board::neighbors(0, 5) {
	// 	println!("{:?}", neighbor);
	// }

	// let mut board = Board::TEST1;
	// board.print();
	// let tests: [(usize, usize); 3] = [(0, 0), (0, 1), (4, 4)];
	// for (row, col) in &tests {
	// 	println!("Verify ({}, {}): {}", row, col, board.verify(*row, *col));
	// }
	// println!("Solved? {}", board.solve());
	// board.print();

	// let mut possibles = PossibleSet::all();
	// println!("{:b}", possibles.inner);
	// possibles.remove(7.into());
	// possibles.remove(7.into());
	// possibles.remove(7.into());
	// possibles.remove(7.into());
	// possibles.remove(3.into());
	// println!("{:b}", possibles.inner);
	// for n in 1..=9 {
	// 	println!("Has {}? {}", n, possibles.contains(n.into()));
	// }

	let mut board = Board::generate();
	board.print();

	println!("Solved? {}", board.solve());
	board.print();
}
