pub mod part1;
pub mod part2;

#[derive(Clone, Debug)]
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

    pub fn file_id(&self) -> Option<u32> {
        match self {
            Self::Free => None,
            Self::File { id } => Some(*id),
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
        #[derive(Debug, Clone)]
        struct BlocksGroup {
            position: u32,
            size: u32,
            block: Block,
        }

        let mut groups = Vec::new();
        let mut current_group = BlocksGroup {
            position: 0,
            size: 0,
            block: Block::File { id: 0 },
        };
        for i in 0..self.blocks.len() {
            current_group = if self.blocks[i].is_free() {
                if current_group.block.is_free() {
                    BlocksGroup {
                        position: current_group.position,
                        size: current_group.size + 1,
                        block: current_group.block,
                    }
                } else {
                    groups.push(current_group.clone());
                    BlocksGroup {
                        position: i as u32,
                        size: 1,
                        block: Block::Free,
                    }
                }
            } else {
                if current_group.block.is_free() {
                    groups.push(current_group.clone());
                    BlocksGroup {
                        position: i as u32,
                        size: 1,
                        block: Block::File { id: self.blocks[i].file_id().unwrap() },
                    }
                } else if current_group.block.file_id().unwrap() == self.blocks[i].file_id().unwrap() {
                    BlocksGroup {
                        position: current_group.position,
                        size: current_group.size + 1,
                        block: current_group.block,
                    }
                } else {
                    groups.push(current_group.clone());
                    BlocksGroup {
                        position: i as u32,
                        size: 1,
                        block: Block::File { id: self.blocks[i].file_id().unwrap() },
                    }
                }
            }
        }
        groups.push(current_group.clone());

        // now, run defragmentation
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
