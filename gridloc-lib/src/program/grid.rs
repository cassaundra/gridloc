use ndarray::Array2;

use std::collections::HashMap;

use crate::*;

type ChunkCoord = (isize, isize);
type ChunkOffset = (usize, usize);

const CHUNK_LENGTH: usize = 8;
// const CHUNK_SIZE: usize = CHUNK_LENGTH * CHUNK_LENGTH;

pub trait Grid: Default {
    fn get(&self, pos: &Position) -> u8;
    fn set(&mut self, pos: &Position, value: u8) -> u8;
}

#[derive(Default)]
pub struct HashGrid {
    chunks: HashMap<ChunkCoord, Chunk>,
    // TODO cache the last used chunk?
}

impl HashGrid {
    fn get_chunk(&self, coord: &ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(coord)
    }

    fn get_chunk_mut(&mut self, coord: &ChunkCoord) -> &mut Chunk {
        self.chunks.entry(*coord)
            .or_default()
    }
}

impl Grid for HashGrid {
    fn get(&self, pos: &Position) -> u8 {
        let (chunk_coords, chunk_offset) = position_to_chunk(pos);

        if let Some(chunk) = self.get_chunk(&chunk_coords) {
            chunk.get(&chunk_offset)
        } else {
            0
        }
    }

    fn set(&mut self, pos: &Position, value: u8) -> u8 {
        let (chunk_coords, chunk_offset) = position_to_chunk(pos);

        let chunk = self.get_chunk_mut(&chunk_coords);
        let old_value = chunk.set(&chunk_offset, value);

        // remove the chunk if it is now empty
        if chunk.is_empty() {
            self.chunks.remove(&chunk_coords);
        }

        old_value
    }
}

struct Chunk {
    cells: Array2<u8>,
    non_zero_count: usize,
}

impl Chunk {
    pub fn get(&self, offset: &ChunkOffset) -> u8 {
        assert!(offset.0 < CHUNK_LENGTH && offset.1 < CHUNK_LENGTH);

        self.cells[[offset.1, offset.0]]
    }

    pub fn set(&mut self, offset: &ChunkOffset, value: u8) -> u8 {
        assert!(offset.0 < CHUNK_LENGTH && offset.1 < CHUNK_LENGTH);

        let old_value = self.cells[[offset.1, offset.0]];
        self.cells[[offset.1, offset.0]] = value;

        if old_value == 0 && value != 0 {
            // changed to non-zero, increment count
            self.non_zero_count += 1;
        } else if old_value != 0 && value == 0 {
            // changed to zero, decrement count
            self.non_zero_count -= 1;
        }

        old_value
    }

    pub fn is_empty(&self) -> bool {
        self.non_zero_count == 0
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            cells: Array2::zeros((CHUNK_LENGTH, CHUNK_LENGTH)),
            non_zero_count: 0,
        }
    }
}

fn position_to_chunk(pos: &Position) -> (ChunkCoord, ChunkOffset) {
    let coord = (pos.x / CHUNK_LENGTH as isize, pos.y / CHUNK_LENGTH as isize);
    let offset = (pos.x.rem_euclid(CHUNK_LENGTH as isize) as usize, pos.y.rem_euclid(CHUNK_LENGTH as isize) as usize);

    (coord, offset)
}
