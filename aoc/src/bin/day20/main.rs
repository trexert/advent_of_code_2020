use multimap::MultiMap;
use ndarray::{s, Array1, Array2};
use std::collections::{HashMap, HashSet};

const PIECE_SIZE: usize = 10;

fn main() {
    let pieces: Vec<_> = include_str!("input.txt")
        .trim()
        .split("\n\n")
        .map(|s| Piece::from_str(s))
        .collect();

    let mapping = build_mapping(&pieces);
    println!("part1: {}", part1(&mapping));
}

fn build_mapping(pieces: &Vec<Piece>) -> MultiMap<Array1<Pixel>, &Piece> {
    pieces
        .iter()
        .flat_map(|piece| {
            piece
                .unflipped_edges
                .iter()
                .chain(piece.flipped_edges.iter())
                .map(move |edge| (edge.clone(), piece))
        })
        .collect()
}

fn part1(mapping: &MultiMap<Array1<Pixel>, &Piece>) -> usize {
    let mut corners: HashSet<&Piece> = HashSet::new();
    let mut seen_pieces: HashMap<&Piece, usize> = HashMap::new();
    mapping.iter_all().for_each(|(_edge, pieces)| {
        match (pieces.len(), seen_pieces.get_mut(pieces[0])) {
            (1, Some(3)) => {
                corners.insert(pieces[0]);
            }
            (1, Some(x)) => *x += 1,
            (1, None) => {
                seen_pieces.insert(pieces[0], 1);
            }
            _ => {}
        };
    });
    assert_eq!(4, corners.len());
    corners.iter().fold(1, |acc, piece| acc * piece.id)
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Piece {
    pub id: usize,
    data: Array2<Pixel>,
    pub unflipped_edges: Vec<Array1<Pixel>>,
    pub flipped_edges: Vec<Array1<Pixel>>,
}

impl Piece {
    pub fn from_str(piece_str: &str) -> Piece {
        let mut input_lines = piece_str.lines();
        let id = input_lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse()
            .unwrap();

        let data = Array2::from_shape_vec(
            (PIECE_SIZE, PIECE_SIZE),
            input_lines
                .flat_map(|line| line.chars().map(|c| Pixel::from_char(c)))
                .collect(),
        )
        .unwrap();

        let unflipped_edges = vec![s![0, ..], s![.., -1], s![-1, ..], s![.., 0]]
            .iter()
            .map(|slice_info| data.slice(slice_info).to_owned())
            .collect();
        let flipped_edges = vec![s![0, ..;-1], s![..;-1, 0], s![-1, ..;-1], s![..;-1, -1]]
            .iter()
            .map(|slice_info| data.slice(slice_info).to_owned())
            .collect();

        Piece {
            id,
            data,
            unflipped_edges,
            flipped_edges,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
enum Pixel {
    HASH,
    DOT,
}

impl Pixel {
    pub fn from_char(c: char) -> Pixel {
        match c {
            '#' => Pixel::HASH,
            '.' => Pixel::DOT,
            _ => panic!("Unexpeted character in piece"),
        }
    }
}
