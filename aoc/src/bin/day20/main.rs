use lazy_static::lazy_static;
use multimap::MultiMap;
use ndarray::{s, Array1, Array2, ArrayView1, ArrayView2};
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

fn part2<'a>(
    pieces: &Vec<Piece>,
    mapping: &'a MultiMap<ArrayView1<'a, Pixel>, &Piece>,
    corners: &Vec<&'a Piece>,
) -> usize {
    let completed_puzzle = build_puzzle(pieces, mapping, corners);
    let monsters = find_monsters(&completed_puzzle);

    completed_puzzle
        .iter()
        .filter(|&&pixel| pixel == Pixel::HASH)
        .count()
        - (monsters * HASHES_PER_MONSTER)
}

fn build_mapping(pieces: &Vec<Piece>) -> MultiMap<ArrayView1<Pixel>, &Piece> {
    pieces
        .iter()
        .flat_map(|piece| {
            piece
                .unflipped_edges
                .iter()
                .chain(piece.flipped_edges.iter())
                .map(move |edge| (edge.view(), piece))
        })
        .collect()
}

fn find_corners<'a>(mapping: &MultiMap<ArrayView1<Pixel>, &'a Piece>) -> Vec<&'a Piece> {
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

fn build_puzzle<'a>(
    pieces: &Vec<Piece>,
    mapping: &'a MultiMap<ArrayView1<'a, Pixel>, &Piece>,
    corners: &Vec<&'a Piece>,
) -> Array2<Pixel> {
    let piece_width = (pieces.len() as f64).sqrt() as usize;
    let pixel_width = piece_width * PIECE_SIZE;
    let mut puzzle = Array2::from_shape_simple_fn((pixel_width, pixel_width), || Pixel::DOT);

    let first_piece = corners[0];
    let left_and_top: Vec<_> = first_piece
        .unflipped_edges
        .iter()
        .enumerate()
        .filter_map(|(i, edge)| {
            if mapping.get_vec(&edge.view()).unwrap().len() == 1 {
                Some((i, edge.view()))
            } else {
                None
            }
        })
        .collect();

    let mut previous_right;
    let mut previous_bottom = if left_and_top[0].0 == 0 && left_and_top[1].0 == 3 {
        left_and_top[0].1.slice(s![..;-1])
    } else {
        left_and_top[1].1.slice(s![..;-1])
    };

    let mut current_left_piece = None;
    let mut current_piece;

    for row in 0..piece_width {
        let top = previous_bottom.slice(s![..;-1]);
        current_left_piece = get_next_piece(mapping, &top, current_left_piece);
        current_piece = current_left_piece;

        let oriented = current_piece.unwrap().oriented_top(&top);
        copy_into(&mut puzzle, &oriented, (row, 0));

        previous_right = current_piece.unwrap().right_side(&top);
        previous_bottom = current_piece.unwrap().opposite_side(&top);

        for column in 1..piece_width {
            let left = previous_right.slice(s![..;-1]);
            current_piece = get_next_piece(mapping, &left, current_piece);

            let oriented = current_piece.unwrap().oriented_left(&left);
            copy_into(&mut puzzle, &oriented, (row, column));

            previous_right = current_piece.unwrap().opposite_side(&left);
        }
    }

    puzzle
}

fn get_next_piece<'a>(
    mapping: &MultiMap<ArrayView1<Pixel>, &'a Piece>,
    side: &ArrayView1<Pixel>,
    current_piece: Option<&Piece>,
) -> Option<&'a Piece> {
    let all_sides = mapping
        .iter_all()
        .filter_map(|(edge, pieces)| if edge == side { Some(pieces) } else { None })
        .next()
        .unwrap();
    let possible_sides: Vec<_> = all_sides
        .iter()
        .filter_map(|piece| match current_piece {
            Some(x) if x == *piece => None,
            _ => Some(*piece),
        })
        .collect();
    possible_sides.into_iter().next()
}

fn copy_into(target: &mut Array2<Pixel>, source_piece: &Array2<Pixel>, position: (usize, usize)) {
    let mut target_area = target.slice_mut(s![
        position.0 * PIECE_SIZE..(position.0 + 1) * PIECE_SIZE,
        position.1 * PIECE_SIZE..(position.1 + 1) * PIECE_SIZE
    ]);

    target_area
        .iter_mut()
        .zip(source_piece.slice(s![1..-1, 1..-1]).iter())
        .for_each(|(tgt, src)| *tgt = *src);
}

const MONSTER_WIDTH: usize = 20;
const MONSTER_HEIGHT: usize = 3;

fn find_monsters(puzzle: &Array2<Pixel>) -> usize {
    let mut found = 0;
    'outer: for mirror in vec![true, false] {
        for rotation_order in 0..4 {
            let transformed_puzzle = if mirror {
                puzzle.mirrored().rotated(rotation_order)
            } else {
                puzzle.rotated(rotation_order)
            };
            found = find_monsters_single_orientation(&transformed_puzzle);
            if found > 0 {
                break 'outer;
            }
        }
    }

    found
}

fn find_monsters_single_orientation(puzzle: &Array2<Pixel>) -> usize {
    let mut found = 0;
    for row in 0..(puzzle.nrows() - MONSTER_HEIGHT) {
        for column in 0..(puzzle.ncols() - MONSTER_WIDTH) {
            found += match_monster(&puzzle.slice(s![
                row..(row + MONSTER_HEIGHT),
                column..(column + MONSTER_WIDTH)
            ])) as usize;
        }
    }
    found
}

fn match_monster(area_to_check: &ArrayView2<Pixel>) -> bool {
    lazy_static! {
        static ref MONSTER: Array2<Pixel> = Array2::from_shape_vec(
            (MONSTER_HEIGHT, MONSTER_WIDTH),
            "\
            ..................#.\
            #....##....##....###\
            .#..#..#..#..#..#...\
            "
            .chars()
            .map(|c| Pixel::from_char(c))
            .collect()
        )
        .unwrap();
    }
    MONSTER
        .iter()
        .zip(area_to_check.iter())
        .all(|(&mon, &img)| !(mon == Pixel::HASH && img == Pixel::DOT))
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

        let unflipped_edges = vec![s![0, ..], s![.., -1], s![-1, ..;-1], s![..;-1, 0]]
            .iter()
            .map(|slice_info| data.slice(slice_info).to_owned())
            .collect();
        let flipped_edges = vec![s![0, ..;-1], s![.., 0], s![-1, ..], s![..;-1, -1]]
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

    fn oriented_top(&self, side: &ArrayView1<Pixel>) -> Array2<Pixel> {
        self.oriented(side, 0)
    }

    fn oriented_left(&self, side: &ArrayView1<Pixel>) -> Array2<Pixel> {
        self.oriented(side, 3)
    }

    fn oriented(&self, side: &ArrayView1<Pixel>, order: usize) -> Array2<Pixel> {
        match (
            self.unflipped_edges.iter().position(|edge| edge == side),
            self.flipped_edges.iter().position(|edge| edge == side),
        ) {
            (Some(x), None) => self.data.rotated((4 + order - x) % 4),
            (None, Some(x)) => self.data.mirrored().rotated((4 + order - x) % 4),
            _ => panic!("Unexpected side result"),
        }
    }

    pub fn right_side(&self, side: &ArrayView1<Pixel>) -> ArrayView1<Pixel> {
        self.relative_side(side, 1)
    }

    pub fn opposite_side(&self, side: &ArrayView1<Pixel>) -> ArrayView1<Pixel> {
        self.relative_side(side, 2)
    }

    fn relative_side(&self, side: &ArrayView1<Pixel>, relative_order: usize) -> ArrayView1<Pixel> {
        match (
            self.unflipped_edges.iter().position(|edge| edge == side),
            self.flipped_edges.iter().position(|edge| edge == side),
        ) {
            (Some(x), None) => self.unflipped_edges[(x + relative_order) % 4].view(),
            (None, Some(x)) => self.flipped_edges[(x + relative_order) % 4].view(),
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
            _ => panic!("Unexpected character in piece"),
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
            2 => (self.shape()[0], self.shape()[1]),
            1 | 3 => (self.shape()[1], self.shape()[0]),
            _ => panic!("Unexpected order"),
        };

        Array2::from_shape_fn(new_shape, |(new_row, new_column)| match order {
            1 => self[[self.nrows() - new_column - 1, new_row]].clone(),
            2 => self[[self.nrows() - new_row - 1, self.ncols() - new_column - 1]].clone(),
            3 => self[[new_column, self.ncols() - new_row - 1]].clone(),
            _ => panic!("Unexpected order"),
        })
    }
    fn mirrored(&self) -> Array2<A> {
        self.slice(s![.., ..;-1]).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::Transformable;
    use ndarray::{arr2, Array2};

    #[test]
    fn rotation1() {
        let initial = arr2(&[[1, 2, 3], [4, 5, 6]]);
        let expected = arr2(&[[4, 1], [5, 2], [6, 3]]);
        assert_eq!(initial.rotated(1), expected);
    }

    #[test]
    fn rotation2() {
        let initial = arr2(&[[1, 2, 3], [4, 5, 6]]);
        let expected = arr2(&[[6, 5, 4], [3, 2, 1]]);
        assert_eq!(initial.rotated(2), expected);
    }

    #[test]
    fn rotation3() {
        let initial = arr2(&[[1, 2, 3], [4, 5, 6]]);
        let expected = arr2(&[[3, 6], [2, 5], [1, 4]]);
        assert_eq!(initial.rotated(3), expected);
    }

    #[test]
    fn mirror() {
        let initial = arr2(&[[1, 2, 3], [4, 5, 6]]);
        let expected = arr2(&[[3, 2, 1], [6, 5, 4]]);
        assert_eq!(initial.mirrored(), expected);
    }

    #[test]
    fn indexing() {
        let a = Array2::from_shape_vec((2, 3), vec![1, 2, 3, 4, 5, 6]).unwrap();
        println!("{:?}", a);
        assert_eq!(a.shape(), &[2, 3]);
        assert_eq!(a.nrows(), 2);
        assert_eq!(a.ncols(), 3);
        assert_eq!(a[[1, 2]], 6);
    }
}
