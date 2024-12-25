#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use glam::IVec2;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
};
use pathfinding::prelude::dijkstra;
use std::collections::HashSet;

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_memory_byte(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(parse_number, tag(","), parse_number)(input)
}

fn parse_memory_bytes(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list0(newline, parse_memory_byte)(input)
}

const MIN_BOUND: i32 = 0;
const MAX_BOUND: i32 = 70;

fn is_within_bounds(pos: IVec2) -> bool {
    pos.cmpge(IVec2::splat(MIN_BOUND)).all() && pos.cmple(IVec2::splat(MAX_BOUND)).all()
}

fn neighbors(pos: &IVec2, corrupted: &HashSet<IVec2>) -> Vec<(IVec2, usize)> {
    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        .iter()
        .map(|&dir| *pos + dir)
        .filter(|&next_pos| {
            is_within_bounds(next_pos) && !corrupted.contains(&next_pos)
        })
        .map(|pos| (pos, 1))
        .collect()
}

fn has_path_to_exit(corrupted: &HashSet<IVec2>) -> bool {
    let start = IVec2::new(0, 0);
    let end = IVec2::new(MAX_BOUND, MAX_BOUND);

    dijkstra(
        &start,
        |pos| neighbors(pos, corrupted),
        |pos| *pos == end,
    ).is_some()
}

fn solve_fst_part(memory_bytes: &Vec<(usize, usize)>) {
    let corrupted: HashSet<IVec2> = memory_bytes
        .iter()
        .take(1024)
        .map(|&(x, y)| IVec2::new(x as i32, y as i32))
        .collect();

    let start = IVec2::new(0, 0);
    let end = IVec2::new(MAX_BOUND, MAX_BOUND);

    let result = dijkstra(
        &start,
        |pos| neighbors(pos, &corrupted),
        |pos| *pos == end,
    );

    match result {
        Some((_, cost)) => {
            println!("First part: {}", cost);
        }
        None => unreachable!(),
    }
}

fn solve_snd_part(memory_bytes: &Vec<(usize, usize)>) {
    let mut corrupted = HashSet::new();
    
    for &(x, y) in memory_bytes.iter() {
        let pos = IVec2::new(x as i32, y as i32);
        _ = corrupted.insert(pos);
        
        if !has_path_to_exit(&corrupted) {
            println!("Second part: {},{}", x, y);
            break;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let (_, memory_bytes) = parse_memory_bytes(&input).map_err(|e| format!("Error: {}", e))?;
    
    solve_fst_part(&memory_bytes);
    solve_snd_part(&memory_bytes);

    Ok(())
}