pub mod part1;
pub mod part2;

#[derive(Clone)]
pub enum Block {
    Free,
    File {
        id: u32,
    },
}

impl Block {
    pub fn is_free(&self) -> bool {
        match self {
            Self::Free => true,
            _ => false,
        }
    }
}

pub struct DiskMap {
    blocks: Vec<Block>,
    first_empty_position: u32,
    last_file_position: u32,
}

impl DiskMap {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            first_empty_position: 0,
            last_file_position: 0,
        }
    }

    pub fn from_blocks(blocks: Vec<Block>) -> Self {
        Self {
            first_empty_position: detect_first_free_position(&blocks, 0).unwrap(),
            last_file_position: detect_last_file_position(&blocks, blocks.len() as u32 - 1).unwrap(),
            blocks,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn defragment(&mut self) {
        while self.first_empty_position < self.last_file_position {
            self.blocks.swap(self.first_empty_position as usize, self.last_file_position as usize);
            self.first_empty_position = match detect_first_free_position(&self.blocks, self.first_empty_position + 1) {
                Some(v) => v,
                None => break,
            };
            self.last_file_position = match detect_last_file_position(&self.blocks, self.last_file_position - 1) {
                Some(v) => v,
                None => break,
            }
        }
    }

    pub fn defragment_contiguous_files(&mut self) {
        // first, build an index
        struct FreeBlocks {
            position: u32,
            size: u32,
        }

        // TODO: continue this
        // let mut free_blocks = Vec::new();
        let mut current_free_blocks: Option<FreeBlocks> = None;
        for i in 0..self.blocks.len() {
            if self.blocks[i].is_free() {
                if let Some(current_blocks) = current_free_blocks.as_mut() {
                    current_blocks.size += 1;
                }
            }
        }
    }

    pub fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(position, block)| match block {
                Block::Free => 0,
                Block::File { id } => *id as u64 * position as u64,
            })
            .sum()
    }
}

fn parse_disk_map(disk_map_str: &str) -> DiskMap {
    let mut disk_map = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;

    for c in disk_map_str.replace("\n", "").chars().into_iter() {
        let size = c.to_digit(10).unwrap();

        for _ in 0..size {
            disk_map.push(if is_file {
                Block::File {
                    id: file_id,
                }
            } else {
                Block::Free
            });
        }

        is_file = if is_file {
            file_id += 1;
            false
        } else {
            true
        };
    }

    DiskMap::from_blocks(disk_map)
}

fn detect_first_free_position(blocks: &[Block], starting_with: u32) -> Option<u32> {
    for i in starting_with as usize..blocks.len() {
        if blocks[i].is_free() {
            return Some(i as u32);
        }
    }
    None
}

fn detect_last_file_position(blocks: &[Block], ending_with: u32) -> Option<u32> {
    for i in (0..(ending_with + 1) as usize).rev() {
        if !blocks[i].is_free() {
            return Some(i as u32);
        }
    }
    None
}
