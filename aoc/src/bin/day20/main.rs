use multimap::MultiMap;
use ndarray::{s, Array1, Array2};
use std::collections::HashMap;

const HASHES_PER_MONSTER: usize = 15;
const PIECE_SIZE_WITH_EDGES: usize = 10;
const PIECE_SIZE: usize = 8;

fn main() {
    let pieces: Vec<_> = include_str!("input.txt")
        .trim()
        .split("\n\n")
        .map(|s| Piece::from_str(s))
        .collect();

    let mapping = build_mapping(&pieces);
    let corners = find_corners(&mapping);
    println!("part1: {}", part1(&corners));
    println!("part2: {}", part2(&pieces, &mapping, &corners))
}

fn part1(corners: &Vec<&Piece>) -> usize {
    corners.iter().fold(1, |acc, piece| acc * piece.id)
}

fn part2(
    pieces: &Vec<Piece>,
    mapping: &MultiMap<Array1<Pixel>, &Piece>,
    corners: &Vec<&Piece>,
) -> usize {
    let completed_puzzle = build_puzzle(pieces, mapping, corners);
    let monsters = find_monsters(&completed_puzzle);

    completed_puzzle
        .iter()
        .filter(|&&pixel| pixel == Pixel::HASH)
        .count()
        - (monsters * HASHES_PER_MONSTER)
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

fn find_corners<'a>(mapping: &MultiMap<Array1<Pixel>, &'a Piece>) -> Vec<&'a Piece> {
    let mut corners: Vec<&Piece> = Vec::new();
    let mut seen_pieces: HashMap<&Piece, usize> = HashMap::new();
    mapping.iter_all().for_each(|(_edge, pieces)| {
        match (pieces.len(), seen_pieces.get_mut(pieces[0])) {
            (1, Some(3)) => {
                corners.push(pieces[0]);
            }
            (1, Some(x)) => *x += 1,
            (1, None) => {
                seen_pieces.insert(pieces[0], 1);
            }
            _ => {}
        };
    });
    assert_eq!(4, corners.len());
    corners
}

fn build_puzzle(
    pieces: &Vec<Piece>,
    mapping: &MultiMap<Array1<Pixel>, &Piece>,
    corners: &Vec<&Piece>,
) -> Array2<Pixel> {
    let piece_width = (pieces.len() as f64).sqrt() as usize;
    let pixel_width = piece_width * PIECE_SIZE;
    let mut puzzle = Array2::from_shape_simple_fn((pixel_width, pixel_width), || Pixel::DOT);

    let mut current_piece = corners[0];
    let left_and_top: Vec<_> = current_piece
        .unflipped_edges
        .iter()
        .enumerate()
        .filter(|(_i, edge)| mapping.get_vec(edge).len() == 1)
        .collect();

    let (left, top) = if left_and_top[0].0 == 1 && left_and_top[1].0 == 2 {
        (left_and_top[1].1, left_and_top[0].1)
    } else {
        (left_and_top[0].1, left_and_top[1].1)
    };

    let (oriented1, mut next_right) = current_piece.oriented_left(left);
    let (oriented2, mut next_bottom) = current_piece.oriented_top(top);

    assert_eq!(oriented1, oriented2);

    let mut oriented = Some(oriented1);

    for row in 0..piece_width {
        for column in 0..piece_width {
            copy_into(&mut puzzle, &oriented.unwrap(), (row, column));

            let mirrored_right = next_right.slice(s![..;-1]).to_owned();
            oriented = if let Some(piece) = mapping
                .get_vec(&mirrored_right)
                .unwrap()
                .iter()
                .filter(|piece| piece != current_piece)
                .next()
            {
                let oriented_result = piece.oriented_left(&mirrored_right);
                next_right = Some(new_oriented)
            } else {
                None
            }
        }
    }

    puzzle
}

fn copy_into(target: &mut Array2<Pixel>, source: &Array2<Pixel>, position: (usize, usize)) {
    let mut target_area = target.slice_mut(s![
        position.0 * PIECE_SIZE..(position.0 + 1) * PIECE_SIZE,
        position.1 * PIECE_SIZE..(position.1 + 1) * PIECE_SIZE
    ]);

    target_area
        .iter_mut()
        .zip(source.iter())
        .for_each(|(tgt, src)| *tgt = *src);
}

fn find_monsters(puzzle: &Array2<Pixel>) -> usize {
    todo!()
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
            (PIECE_SIZE_WITH_EDGES, PIECE_SIZE_WITH_EDGES),
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

    fn oriented_top(&self, side: &Array1<Pixel>) -> (Array2<Pixel>, &Array1<Pixel>) {
        self.oriented(side, 0)
    }

    fn oriented_left(&self, side: &Array1<Pixel>) -> (Array2<Pixel>, &Array1<Pixel>) {
        self.oriented(side, 3)
    }

    fn oriented(&self, side: &Array1<Pixel>, order: usize) -> (Array2<Pixel>, &Array1<Pixel>) {
        match (
            self.unflipped_edges.iter().position(|edge| edge == side),
            self.flipped_edges.iter().position(|edge| edge == side),
        ) {
            (Some(x), None) => {
                let new_data = self.data.rotated((4 + order - x) % 4);
                (new_data, &self.unflipped_edges[(x + 2) % 4])
            }
            (None, Some(x)) => {
                let new_data = self.data.mirrored().rotated((4 + order - x) % 4);
                (new_data, &self.flipped_edges[(x + 2) % 4])
            }
            _ => panic!("Unexpected side result"),
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

trait Transformable {
    fn rotated(&self, order: usize) -> Self;
    fn mirrored(&self) -> Self;
}

impl<A: Clone> Transformable for Array2<A> {
    fn rotated(&self, order: usize) -> Array2<A> {
        if order == 0 {
            return self.clone();
        }

        let new_shape = match order {
            2 => (self.shape()[1], self.shape()[0]),
            1 | 3 => (self.shape()[0], self.shape()[1]),
            _ => panic!("Unexpected order"),
        };

        Array2::from_shape_fn(new_shape, |(new_row, new_column)| match order {
            1 => self[[new_column, self.shape()[0] - new_row]].clone(),
            2 => self[[self.shape()[0] - new_row, self.shape()[1] - new_column]].clone(),
            3 => self[[self.shape()[1] - new_column, new_row]].clone(),
            _ => panic!("Unexpected order"),
        })
    }
    fn mirrored(&self) -> Array2<A> {
        let width = self.shape()[1];
        Array2::from_shape_fn(self.raw_dim(), |(row, column)| {
            self[[row, width - column]].clone()
        })
    }
}
