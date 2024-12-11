struct Disk {
    checksum: i64,
    pos: i64,
}

impl Disk {
    fn add_file(&mut self, id: i16, size: i16) {
        let size_64 = i64::from(size);
        self.checksum += i64::from(id) * file_checksum(self.pos, size_64);
        self.pos += size_64;
    }
}

fn to_number(value: char) -> i16 {
    i16::try_from(value.to_digit(10).unwrap()).unwrap()
}

fn file_checksum(idx: i64, size: i64) -> i64 {
    // S = n(a + l)/2
    size * (2 * idx + (size - 1)) / 2
}

type FsEntry = (Option<i16>, i16);

fn find_file_to_move(fs: &Vec<FsEntry>) -> Option<((usize, (i16, i16)), (usize, i16))> {
    for (idx, right) in fs.iter().enumerate().rev() {
        if let Some(right_id) = right.0 {
            let free_space_result = fs
                .iter()
                .enumerate()
                .take(idx)
                .find(|(_, v)| v.0.is_none() && v.1 >= right.1);

            if let Some((free_idx, (_, free_space))) = free_space_result {
                return Some(((idx, (right_id, right.1)), (free_idx, *free_space)));
            }
        }
    }

    None
}

pub fn run(data: &String) {
    let (input, _) = data.split_once("\n").unwrap();
    let mut fs = Vec::<Option<i16>>::with_capacity(data.len());
    let mut defrag_fs = Vec::<FsEntry>::with_capacity(data.len());
    let mut checksum: usize = 0;

    for (idx, size) in input.chars().map(to_number).enumerate() {
        if idx % 2 == 0 {
            defrag_fs.push((Some(i16::try_from(idx).unwrap() / 2), size));
        } else {
            defrag_fs.push((None, size));
        }

        for _ in 0..size {
            if idx % 2 == 0 {
                fs.push(Some(i16::try_from(idx / 2).unwrap()));
            } else {
                fs.push(None);
            }
        }
    }

    let mut new_fs = fs.clone();
    for (i, content) in fs.iter().enumerate() {
        if content.is_none() {
            if let Some(r) = new_fs.iter().rev().position(|v| v.is_some()) {
                let r_idx = fs.len() - r - 1;
                if r_idx > i {
                    new_fs.swap(i, r_idx);
                } else {
                    break;
                }
            }
        }
    }

    for (pos, c) in new_fs.into_iter().enumerate() {
        if c.is_some() {
            checksum += usize::try_from(c.unwrap()).unwrap() * pos;
        }
    }

    loop {
        if let Some(((file_idx, file), (space_idx, space_size))) = find_file_to_move(&defrag_fs) {
            if file.1 < space_size {
                defrag_fs[space_idx] = (Some(file.0), file.1);
                defrag_fs[file_idx] = (None, file.1);
                defrag_fs.insert(space_idx + 1, (None, space_size - file.1));
            } else {
                defrag_fs.swap(file_idx, space_idx);
            }
        } else {
            break;
        }
    }

    let mut disk = Disk {
        checksum: 0,
        pos: 0,
    };
    for c in defrag_fs.into_iter() {
        if let Some(id) = c.0 {
            disk.add_file(id, c.1);
        } else {
            disk.pos += i64::from(c.1);
        }
    }

    println!("{:?}", checksum);
    // 15779604522443 high
    println!("{:?}", disk.checksum);
}
