//! # 扫雷
//! ## 全局
//! 0. 有10个雷
//!   - 当未反转的格子为10个时胜利
//! 1. 9*9个格子

use std::fmt::*;

pub struct Minewreeper {
    blocks: Vec<Block>,
}

pub trait Model {
    /// generate a table for game. 从棋盘的左上角开始从左往右生成
    fn init() -> Minewreeper;
    fn renew(&mut self);
    fn flip_around(&mut self, center: &(u8, u8));
}

impl Model for Minewreeper {
    fn init() -> Minewreeper {
        use crate::{ thread_rng, Rng };
        let mut minewreeper = Minewreeper { blocks: Vec::new(), };

        let mut mine = Vec::new();
        // random create landmines
        let mut rng = thread_rng();
        for _ in 0..=10 {
            let x: u8 = rng.gen_range(0..9);
            let y: u8 = rng.gen_range(0..9);
            mine.push((x, y));
        }

        // Setting blocks
        for y1 in (0..9).rev() {
            for x1 in 0..9 {
                let position = (x1, y1);
                let mut is_boom = false;
                let index = minewreeper.blocks.len();
                for pos in &mine {
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
        minewreeper.crutalmovment();
        minewreeper
    }

    fn renew(&mut self) {
        use crate::{ thread_rng, Rng };
        let mut mine = Vec::new();
        // random mine
        let mut rng = thread_rng();
        for _ in 0..=10 {
            let x: u8 = rng.gen_range(0..9);
            let y: u8 = rng.gen_range(0..9);
            mine.push((x, y));
        }
        *self = Minewreeper::init();
        self.crutalmovment();
    }

    /// 递归翻转周围的格子，若打开的格子上数字不为 0 递归收敛
    fn flip_around(&mut self, center: &(u8, u8)) {
        let &(x, y) = center;
        let center_idx = Minewreeper::get_block_idx(&(Some(x), Some(y)));
        // 如果参数中的格子状态为未翻转开则可以翻转
        // 打开后若有数字不为0(周围有雷)，则不继续打开
        match center_idx {
            Some(idx) => {
                let block = &mut self.blocks[idx];
                let num = block.get_text();
                let flag = block.get_flag();
                if flag == &BlockFlag::Normal {
                    block.set_flag(BlockFlag::Selected);
                }
                if num != 0 {
                    return;
                }
            }
            None => {
                return;
            }
        }
        // 找到周围的格子并重复上面的内容
        let neighbors = Minewreeper::around(center);
        for neighbor in neighbors {
            if let Some(idx) = neighbor {
                let block = &self.blocks[idx];
                if block.get_flag() == &BlockFlag::Normal {
                    let &(x, y) = &block.point;
                    self.flip_around(&(x, y));
                }
            }
        }
    }

}

impl Minewreeper {
    /// 获取周围的格子的 index
    pub fn around(point: &(u8, u8)) -> Vec<Option<usize>> {
        let mut neighborhood: Vec<Option<usize>> = Vec::new();
        let &(x, y) = point;
        let cur_pos_x_add = x.checked_add(1);
        let cur_pos_x_sub = x.checked_sub(1);
        let cur_pos_y_add = y.checked_add(1);
        let cur_pos_y_sub = y.checked_sub(1);
        let e = Minewreeper::get_block_idx(&(cur_pos_x_add, Some(y)));
        let s = Minewreeper::get_block_idx(&(Some(x), cur_pos_y_sub));
        let n = Minewreeper::get_block_idx(&(Some(x), cur_pos_y_add));
        let w = Minewreeper::get_block_idx(&(cur_pos_x_sub, Some(y)));
        let en = Minewreeper::get_block_idx(&(cur_pos_x_add, cur_pos_y_add));
        let es = Minewreeper::get_block_idx(&(cur_pos_x_add, cur_pos_y_sub));
        let wn = Minewreeper::get_block_idx(&(cur_pos_x_sub, cur_pos_y_add));
        let ws = Minewreeper::get_block_idx(&(cur_pos_x_sub, cur_pos_y_sub));
        neighborhood.push(e);
        neighborhood.push(s);
        neighborhood.push(n);
        neighborhood.push(w);
        neighborhood.push(en);
        neighborhood.push(es);
        neighborhood.push(wn);
        neighborhood.push(ws);
        neighborhood
    }

    /// 一个周围雷数计数器， 棋盘生成时默认所有 block 的 counter 为 0， 用过此方法遍历所有 block 更新 counter
    #[inline]
    fn crutalmovment(&mut self) {
        let mut unsafe_blocks = Vec::new();
        for b in &self.blocks {
            if b.is_boom() {
                let cur_pos = &b.point;
                unsafe_blocks = [unsafe_blocks, Minewreeper::around(cur_pos)].concat();
            }
        }
        for unsafe_idx in unsafe_blocks {
            if let Some(idx) = unsafe_idx {
                self.blocks[idx].counter += 1;
            }
        }
    }

    pub fn get_block_idx(pos: &(Option<u8>, Option<u8>)) -> Option<usize> {
        match pos {
            (None, _) | (_, None) => return None,
            (Some(x), Some(y)) => {
                if x < &0 || x > &8 {
                    return None;
                }
                if y < &0 || y > &8 {
                    return None;
                }
                return Some((x + (8 - y) * 9) as usize);
            }
        }
    }
}

/// 把棋盘转化为 String
impl Display for Minewreeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = Vec::new();
        for i in 0..self.blocks.len() {
            result.push(format!("{}", self.blocks[i]));
        }
        for (count, res) in result.iter().enumerate() {
            write!(f, "{}", res)?;
        }
        write!(f, "")
    }
}

/// #[cfg(test)] 中使用的 println!("{:?}", board), 格式化用于显示在 console 中的棋盘
impl Debug for Minewreeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = Vec::new();
        for i in 0..self.blocks.len() {
            result.push(format!("{:?}", self.blocks[i]));
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
    counter: u8,
}

impl Block {
    pub fn new(position: (u8, u8), is_boom: bool, index: usize) -> Self {
        Self {
            point: position,
            flag: BlockFlag::Normal,
            is_boom,
            counter: 0,
            // near_block: test,
        }
    }

    pub fn set_counter(&mut self, t: u8) {
        if t > 8 {
            println!("error! text only small than 9");
            std::process::exit(101);
        }
        self.counter = t;
    }

    pub fn is_boom(&self) -> bool {
        self.is_boom
    }

    pub fn set_flag(&mut self, flag: BlockFlag) {
        self.flag = flag;
    }

    pub fn get_text(&self) -> u8 {
        self.counter
    }

    pub fn get_flag(&self) -> &BlockFlag {
        &self.flag
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.flag {
            BlockFlag::Selected => match self.is_boom {
                true => return write!(f, "✹"),
                false => return write!(f, "{}", self.counter),
            },
            BlockFlag::Flaged => return write!(f, "?"),
            BlockFlag::Normal => return write!(f, "■"),
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.flag {
            BlockFlag::Selected => match self.is_boom {
                true => return write!(f, "✹"),
                false => return write!(f, "{}", self.counter),
            },
            BlockFlag::Flaged => return write!(f, "?"),
            BlockFlag::Normal => return write!(f, "■"),
        }
    }
}

/// 格子的种类
#[derive(PartialEq)]
enum BlockFlag {
    /// 已翻转
    Selected,
    /// 标记为问号
    Flaged,
    /// 未翻转
    Normal,
}

///  这个还没用上
#[derive(PartialEq)]
enum Message {
    Boom,
    Playing,
    Win,
    Renew,
}

impl Message {
    fn send(&mut self, status: Message) {
        *self = status;
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use super::{ Model, Minewreeper };
    #[test]
    fn crutalmovment_works() {
        let mut mine = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..=10 {
            let x: u8 = rng.gen_range(0..9);
            let y: u8 = rng.gen_range(0..9);
            mine.push((x, y));
        }
        let mut board = Minewreeper::init();
        println!("{:?}", board);
    }

    #[test]
    fn turn_works() {
        let mut mine = Vec::new();
        // random mine
        let mut rng = thread_rng();
        for _ in 0..=10 {
            let x: u8 = rng.gen_range(0..9);
            let y: u8 = rng.gen_range(0..9);
            mine.push((x, y));
        }

        let mut board = Minewreeper::init();
        board.flip_around(&(4,4));
        println!("{:?}", board);
    }
}
