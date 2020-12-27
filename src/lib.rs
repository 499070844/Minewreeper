//! # 扫雷
//! ## 全局
//! 0. 有10个雷
//!   - 当未反转的格子为10个时胜利
//! 1. 9*9个格子
//!   - 如何表示坐标
//! 2. 如何表示雷
//! 3. 如何表示安全反转
//! 4. 反转后还要在上面显示数字
//! 5. 如何连带其他格子反转

use std::{collections::HashMap, fmt::*};

struct Minewreeper {
    /// 一共81个
    total: u8,
    last: u8,
    /// when
    /// `last` == 10
    ///
    /// `is_wine` == true
    is_win: bool,
    blocks: Vec<Block>,
}

impl Minewreeper {
    /// generate a table for game. 从棋盘的左上角开始从左往右生成
    pub fn init(mine_pos: Vec<(u8, u8)>) -> Self {
        let mut minewreeper = Minewreeper {
            total: 81,
            last: 10,
            is_win: false,
            blocks: Vec::new(),
        };

        for y1 in (0..9).rev() {
            for x1 in 0..9 {
                let position = (x1, y1);
                let mut is_boom = false;
                let index = minewreeper.blocks.len();
                for pos in &mine_pos {
                    is_boom = *pos == position;
                    if is_boom {
                        break;
                    }
                }
                if is_boom {
                    minewreeper
                        .blocks
                        .push(Block::new(position, is_boom, index));
                } else {
                    minewreeper
                        .blocks
                        .push(Block::new(position, is_boom, index));
                }
            }
        }
        minewreeper
    }

    pub fn is_init(&self) -> bool {
        self.blocks.len() > 0
    }

    pub fn crutalmovment(&mut self) {
        let mut unsafe_blocks = Vec::new();
        for b in &self.blocks {
            if b.is_boom {
                let cur_pos = &b.point;
                let e = Minewreeper::get_block_idx(&(cur_pos.0 + 1, cur_pos.1));
                let s = Minewreeper::get_block_idx(&(cur_pos.0, cur_pos.1 - 1));
                let n = Minewreeper::get_block_idx(&(cur_pos.0, cur_pos.1 + 1));
                let w = Minewreeper::get_block_idx(&(cur_pos.0 - 1, cur_pos.1));
                let en = Minewreeper::get_block_idx(&(cur_pos.0 + 1, cur_pos.1 + 1));
                let es = Minewreeper::get_block_idx(&(cur_pos.0 + 1, cur_pos.1 - 1));
                let wn = Minewreeper::get_block_idx(&(cur_pos.0 - 1, cur_pos.1 + 1));
                let ws = Minewreeper::get_block_idx(&(cur_pos.0 - 1, cur_pos.1 - 1));
                unsafe_blocks.push(e);
                unsafe_blocks.push(s);
                unsafe_blocks.push(n);
                unsafe_blocks.push(w);
                unsafe_blocks.push(en);
                unsafe_blocks.push(es);
                unsafe_blocks.push(wn);
                unsafe_blocks.push(ws);
            }
        }
        for unsafe_idx in unsafe_blocks {
            if let Some(idx) = unsafe_idx {
                self.blocks[idx].text += 1;
            }
        }
    }

    pub fn get_block_idx(pos: &(u8, u8)) -> Option<usize> {
        println!("{:?}", pos);
        if pos.0 < 0 || pos.0 > 8 { return None }
        if pos.1 < 0 || pos.1 > 8 { return None }
        Some((pos.0 + (8 - pos.1) * 9) as usize)
    }
}

impl Display for Minewreeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = Vec::new();
        for i in 0..self.blocks.len() {
            result.push(format!("{}", self.blocks[i]));
        }
        for (count, res) in result.iter().enumerate() {
            if count % 9 == 0 {
                write!(f, "\r\n")?;
            }
            write!(f, "{} ", res)?;
        }
        write!(f, "\r\n")
    }
}

struct Block {
    /// (x, y)
    ///
    /// x <= 8
    ///
    /// y <= 8
    point: (u8, u8),
    flag: BlockFlag,
    is_boom: bool,
    // near_block: [NearBlock; 8],
    text: u8,
}

impl Block {
    pub fn new(position: (u8, u8), is_boom: bool, index: usize) -> Self {
        Self {
            point: position,
            flag: BlockFlag::Normal,
            is_boom,
            text: 0,
            // near_block: test,
        }
    }

    pub fn set_text(&mut self, t: u8) {
        if t > 8 {
            println!("error! text only small than 9");
            std::process::exit(101);
        }
        self.text = t;
    }
}


impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.is_boom {
            true => write!(f, "*"),
            false => write!(f, "{}", self.text),
        }
    }
}

enum BlockFlag {
    Selected,
    Flaged,
    Normal,
    Marked,
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use crate::Minewreeper;

    #[test]
    fn it_works() {
        let mut mine = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..=10 {
            let x: u8 = rng.gen_range(0..9);
            let y: u8 = rng.gen_range(0..9);
            mine.push((x, y));
        }
        let mut board = Minewreeper::init(mine);
        print!("{}", board);
        print!("\r\n");
        board.crutalmovment();
        println!("{}", board);
    }
}
