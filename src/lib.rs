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

use std::fmt::*;

struct Minewreeper {
    /// 一共81个
    total: u8,
    last: u8,
    /// when
    /// `last` == 10
    ///
    /// `is_wine` == true
    is_win: bool,
    block: Vec<Block>,
}

impl Minewreeper {
    /// generate a table for game. 从棋盘的左上角开始从左往右生成
    pub fn init(mine_pos: Vec<(u8, u8)>) -> Self {
        let mut minewreeper = Minewreeper {
            total: 81,
            last: 10,
            is_win: false,
            block: Vec::new(),
        };

     for y1 in (0..9).rev() {
             for x1 in 0..9 {
                let position = (x1, y1);
                let mut is_boom = false;
                for pos in &mine_pos {
                    is_boom = *pos == position;
                    if is_boom { break; }
                }
                if is_boom {
                    minewreeper.block.push(Block::new(position, is_boom));
                } else {
                    minewreeper.block.push(Block::new(position, is_boom));
                }
            }
        }
        println!("init success");
        minewreeper
    }

    pub fn is_init(&self) -> bool { self.block.len() > 0 }
}

impl Display for Minewreeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = Vec::new();
        for i in 0..self.block.len() {
            result.push(format!("{}", self.block[i]));
        }
        println!("{}",result.len());
        for (count, res) in result.iter().enumerate() {
            if count % 9 == 0 { write!(f, "\r\n")?; }
            write!(f, "{} ", res)?;
        }
        write!(f, "\r\n")
    }
}

struct Block {
    /// (x, y)
    ///
    /// x <= 9
    ///
    /// y <= 9
    point: (u8, u8),
    flag: BlockFlag,
    is_boom: bool,
    // near_block: [NearBlock; 8],
    text: u8,
}
enum NearBlock {
    E(Option<Box<Block>>),
    N(Option<Box<Block>>),
    W(Option<Box<Block>>),
    S(Option<Box<Block>>),
    EN(Option<Box<Block>>),
    ES(Option<Box<Block>>),
    WN(Option<Box<Block>>),
    WS(Option<Box<Block>>),
}

impl Block {
    pub fn new(position: (u8, u8), is_boom: bool) -> Self {
        Self {
            point: position,
            flag: BlockFlag::Normal,
            is_boom,
            text: 0,
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
    use rand::{ thread_rng, Rng };

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
        print!("{}", Minewreeper::init(mine))
    }
}
