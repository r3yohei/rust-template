#![allow(non_snake_case, unused)]

use proconio::{*, marker::*};
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use std::cmp::*;
use std::collections::*;
use itertools::*;
use superslice::Ext;

/// 定数
const INF: i32 = 1_000_000_000;
const TL: f64 = 1.9;
const H: usize = 10;
const W: usize = 10;
const DIJ: [(usize, usize); 4] = [(0, !0), (0, 1), (!0, 0), (1, 0)];
const DIR: [char; 4] = ['L', 'R', 'U', 'D'];

/// 入力の受け取り
#[derive(Clone)]
struct Input {
    s: (usize, usize),
    board: Vec<Vec<usize>>,
}

/// 盤面の状態を保持する
#[derive(Clone)]
struct State {
    turn: usize,
    end_turn: usize,
    pos: (usize, usize),
    seen: Vec<Vec<bool>>,
    output: String,
    game_score: i32,
    evaluated_score: i32,
}

impl State {
    pub fn new(input: &Input, end_turn: usize, pos: (usize, usize)) -> Self {
        Self {
            turn: 0,
            end_turn: end_turn,
            pos: pos,
            seen: vec![vec![false; W]; H],
            output: String::new(),
            game_score: 0,
            evaluated_score: 0,
        }
    }

    /// 評価関数
    pub fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }

    /// ゲームの終了判定
    pub fn is_done(&self) -> bool {
        self.turn == self.end_turn
    }

    /// 指定したactionでゲームを1ターン進める
    pub fn advance(&mut self, input: &Input, action: usize) {
        self.turn += 1;
        self.pos.0 = self.pos.0.wrapping_add(DIJ[action].0);
        self.pos.1 = self.pos.1.wrapping_add(DIJ[action].1);
        self.seen[self.pos.0][self.pos.1] = true;
        self.output.push(DIR[action]);
        self.game_score += input.board[self.pos.0][self.pos.1];
    }

    /// 現在の状況でプレイヤーが可能な行動を全て取得する
    pub fn legalActions(&self, input: &Input) -> Vec<usize> {
        let mut actions = vec![];
        for action in 0..4 {
            let ni = self.pos.0.wrapping_add(DIJ[action].0);
            let nj = self.pos.1.wrapping_add(DIJ[action].1);
            if ni < H && nj < W && !self.seen[ni][nj] {
                actions.push(action);
            }
        }
        actions
    }

    /// 現在のゲーム状況を標準エラー出力に出力する
    pub fn toString(&self, input: &Input) {
        todo!();
    }
}

/// 探索時のソート用に評価を比較する
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.evaluated_score == other.evaluated_score
    }
}
impl Eq for State {} // ここは空でOK
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.evaluated_score.partial_cmp(&other.evaluated_score)
    }
}

/// 現在時刻を返す
pub fn get_time() -> f64 {
	static mut STIME: f64 = -1.0;
	let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
	let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
	unsafe {
		if STIME < 0.0 {
			STIME = ms;
		}
		// ローカル環境とジャッジ環境の実行速度差はget_timeで吸収しておくと便利
		#[cfg(feature="local")]
		{
			(ms - STIME) * 10.0
		}
		#[cfg(not(feature="local"))]
		{
			(ms - STIME)
		}
	}
}


/// 入力をもとに答えを返す
fn solve(input: &Input, state: &State) -> i32 {
    todo!();
}


fn main() {
    get_time();
    let mut rng = rand_pcg::Pcg64Mcg::new(42);
    input! {
        s: (usize, usize),
        board: [[usize; W]; H],
    }
    let input = Input{s, board};
    let state = State::new(&input, !0, (0, 0));
    let score = solve(&input, &state);

    eprintln!("score = {}", score);
    eprintln!("time = {:.3}", get_time());
}
