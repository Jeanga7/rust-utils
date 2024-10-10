#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32, 
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank_diff = (self.position.rank - other.position.rank).abs();
        let file_diff = (self.position.file - other.position.file).abs();

        self.position.rank == other.position.rank || // même rangée
        self.position.file == other.position.file || // même colonne
        rank_diff == file_diff // diagonale 
    }
}

