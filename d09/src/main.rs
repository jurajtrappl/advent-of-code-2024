#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

#[derive(Clone)]
enum Block {
    File(u32),
    FreeSpace,
}

#[derive(Clone)]
struct FileSystem {
    blocks: Vec<Block>,
}

impl FileSystem {
    fn checksum(&self) -> u64 {
        let mut result = 0;
        for (pos, block) in self.blocks.iter().enumerate() {
            if let Block::File(id) = block {
                result += pos as u64 * (*id as u64);
            }
        }
        result
    }

    fn compact(&mut self) {
        let mut left = 0;
        let mut right = self.blocks.len() - 1;

        while left < right {
            while left < right && !matches!(self.blocks[left], Block::FreeSpace) {
                left += 1;
            }

            while left < right && !matches!(self.blocks[right], Block::File(_)) {
                right -= 1;
            }

            if left < right {
                self.blocks.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
    }

    fn compact_v2(&mut self) {
        for file_id in (0..=self.get_max_file_id()).rev() {
            if let Some((start, end)) = self.find_file_bounds(file_id) {
                let file_size = end - start + 1;

                let mut best_space = None;
                for i in 0..start {
                    if matches!(self.blocks[i], Block::FreeSpace) {
                        let space = self.find_contiguous_free_space(i);
                        if space >= file_size {
                            best_space = Some(i);
                            break;
                        }
                    }
                }

                if let Some(target_start) = best_space {
                    let file_blocks: Vec<Block> = self.blocks[start..=end].iter().cloned().collect();

                    for i in start..=end {
                        self.blocks[i] = Block::FreeSpace;
                    }

                    for (offset, block) in file_blocks.into_iter().enumerate() {
                        self.blocks[target_start + offset] = block;
                    }
                }
            }
        }
    }

    fn find_contiguous_free_space(&self, from: usize) -> usize {
        let mut count = 0;
        for block in self.blocks[from..].iter() {
            if matches!(block, Block::FreeSpace) {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    fn find_file_bounds(&self, file_id: u32) -> Option<(usize, usize)> {
        let (mut start, mut end) = (None, None);

        for (i, block) in self.blocks.iter().enumerate() {
            if let Block::File(id) = block {
                if *id == file_id {
                    if start.is_none() {
                        start = Some(i);
                    }
                    end = Some(i);
                }
            }
        }

        match (start, end) {
            (Some(s), Some(e)) => Some((s, e)),
            _ => None
        }
    }

    fn get_max_file_id(&self) -> u32 {
        let mut max_file_id = 0;
        for block in self.blocks.iter() {
            if let Block::File(id) = block {
                max_file_id = max_file_id.max(*id);
            }
        }
        max_file_id
    }
}

fn build_filesystem(disk_map: String) -> FileSystem {
    let mut fs = FileSystem { blocks: vec![] };
    let mut file_id: u32 = 0;
    for (i, c) in disk_map.chars().enumerate() {
        let num = c.to_digit(10).unwrap() as usize;
        
        if i % 2 == 0 {
            for _ in 0..num {
                fs.blocks.push(Block::File(file_id))
            }

            file_id += 1;
        } else {
            for _ in 0..num {
                fs.blocks.push(Block::FreeSpace)
            }
        }
    }

    fs
}

fn main() {
    let disk_map = aoe::read_input_file("input").unwrap();

    let mut fs = build_filesystem(disk_map);
    let mut fs_2 = fs.clone();

    fs.compact();
    fs_2.compact_v2();
    
    println!("First part: {}, Second part: {}", fs.checksum(), fs_2.checksum());
}