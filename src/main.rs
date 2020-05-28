#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell {
	pub inner: u16
}
impl std::ops::Add for Cell {
	type Output = Self;
	fn add(mut self, other: Self) -> Self {
		self.inner |= other.inner;
		self
	}
}
impl std::ops::Sub for Cell {
	type Output = Self;
	fn sub(mut self, other: Self) -> Self {
		self.inner &= !other.inner;
		self
	}
}
impl Default for Cell {
	fn default() -> Self {
		Self::A + Self::B + Self::C + Self::D + Self::E + Self::F + Self::G + Self::H + Self::I
	}
}
impl Cell {
	pub const A: Cell = Cell { inner: 1 << 0 };
	pub const B: Cell = Cell { inner: 1 << 1 };
	pub const C: Cell = Cell { inner: 1 << 2 };
	pub const D: Cell = Cell { inner: 1 << 3 };
	pub const E: Cell = Cell { inner: 1 << 4 };
	pub const F: Cell = Cell { inner: 1 << 5 };
	pub const G: Cell = Cell { inner: 1 << 6 };
	pub const H: Cell = Cell { inner: 1 << 7 };
	pub const I: Cell = Cell { inner: 1 << 8 };

	pub fn pick(self) -> Self {
		if self.inner & Self::A.inner != 0 {
			Self::A
		} else if self.inner & Self::B.inner != 0 {
			Self::B
		} else if self.inner & Self::C.inner != 0 {
			Self::C
		} else if self.inner & Self::D.inner != 0 {
			Self::D
		} else if self.inner & Self::E.inner != 0 {
			Self::E
		} else if self.inner & Self::F.inner != 0 {
			Self::F
		} else if self.inner & Self::G.inner != 0 {
			Self::G
		} else if self.inner & Self::H.inner != 0 {
			Self::H
		} else if self.inner & Self::I.inner != 0 {
			Self::I
		} else {
			unreachable!()
		}
	}
}
#[derive(Debug)]
struct Board {
	pub board: [[Cell; 9]; 9]
}
impl Board {
	pub fn generate() -> Self {
		let mut ret = Self {
			board: [[Cell::default(); 9]; 9]
		};

		ret
	}
	pub fn print(&self) {
		for row in self.board.iter() {
			for cell in row.iter() {
				print!("{:b} ", cell.inner);
			}
			print!("\n");
		}
	}
}

fn main() {
	let board = Board::generate();
	board.print();
	
	for (r, c, cell) in board.board.iter().enumerate().flat_map(|(r, row)| row.iter().enumerate().map(move |(c, cell)| (r, c, cell))) {
		println!("Row: {} Column: {} Cell: {:?}", r, c, cell);
	}
}
