use std::{collections::HashSet, io::BufRead, ops::{Index, IndexMut}};

use itertools::Itertools;
use nalgebra::{matrix, Matrix3, Point3};
use regex::Regex;
#[derive(Clone)]
struct Tile {
    id: i64,
    dimension: Point3::<i32>,
    on: HashSet<Point3::<i32>>,
    transformation: Matrix3<i32>,
    transformed: Option<HashSet<Point3::<i32>>>,
}
impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile {}:", self.id).unwrap();
        (0..self.dimension.y).for_each(|y| {
            (0..self.dimension.x).for_each(|x| { write!(f, "{}", self[(x,y)]).unwrap(); } );
            writeln!(f, "").unwrap();
        });
        Ok(())
    }
}
impl Tile {
    fn from_string(s: Vec<String>) -> Tile {
        let start_re = Regex::new(r#"Tile (?P<id>[0-9]+):"#).unwrap();
        let mut result = Tile {
            id: start_re.captures(&s[0]).unwrap().name("id").unwrap().as_str().parse::<i64>().unwrap(),
            dimension: Point3::new(s[1].len() as i32, s[1].len() as i32, 1),
            on: s[1..s[1].len() + 1]
                .into_iter()
                .enumerate()
                .map(move |(y,line)| line
                    .chars()
                    .into_iter()
                    .enumerate()
                    .map(move |(x, ch)| if ch == '#' { Some(Point3::new(x.clone() as i32, y.clone() as i32, 1)) } else { None })
                    .filter(|c| c.is_some() )
                )
                .flatten()
                .fold(HashSet::new(), |mut hs, c| { hs.insert(c.unwrap()); hs }),
            transformation: matrix![1, 0, 0;
                                    0, 1, 0;
                                    0, 0, 1],
            transformed: None,
        };
        result.transform();
        result
    }
    fn transform(&mut self) -> () {
        self.transformed = Some(self.on
            .clone()
            .into_iter()
            .map(|coord| self.transformation * coord )
            .fold(HashSet::new(), |mut hs, coord| { hs.insert(coord); hs })
        );
    }
    fn set_transformation(&mut self, matrix: Matrix3<i32>) -> () {
        self.transformation = matrix;
        self.transform();
    }
    fn transformations(&self) -> Vec<Matrix3<i32>> {
        let identity = matrix![1, 0, 0;
                                                   0, 1, 0;
                                                   0, 0, 1];
        let rotation = matrix![1, 0, 0;
                                                   0, 1, self.dimension.y-1;
                                                   0, 0, 1]
                                                   *
                                                   matrix![0, 1, 0;
                                                           -1,  0, 0;
                                                           0,  0, 1];
        let flipv = matrix![1, 0, 0;
                                                   0, 1, self.dimension.y-1;
                                                   0, 0, 1]
                                                   *
                                                   matrix![1, 0, 0;
                                                           0,  -1, 0;
                                                           0,  0, 1];
        let fliph = matrix![1, 0, self.dimension.x-1;
                                                   0, 1, 0;
                                                   0, 0, 1]
                                                   *
                                                   matrix![-1, 0, 0;
                                                           0,  1, 0;
                                                           0,  0, 1];
        [ identity, rotation, rotation * rotation, rotation * rotation * rotation].into_iter()
            .map( |r| [ identity, flipv, fliph, flipv * fliph ].map(|f| f * r ))
            .flatten()
            .collect()
    }
    fn left(&self) -> String {
        (0..self.dimension.y)
            .map(|y| self[(0,y)])
            .join("")
    }
    fn right(&self) -> String {
        (0..self.dimension.y)
            .map(|y| self[(self.dimension.x-1,y)])
            .join("")
    }
    fn top(&self) -> String {
        (0..self.dimension.x)
            .map(|x| self[(x,0)])
            .join("")
    }
    fn bottom(&self) -> String {
        (0..self.dimension.x)
            .map(|x| self[(x,self.dimension.y-1)])
            .join("")
    }
}
impl Index<(i32,i32)> for Tile {
    type Output = char;

    fn index(&self, coord: (i32, i32) ) -> &Self::Output {
        match self.transformed.as_ref().unwrap().get(&Point3::new(coord.0, coord.1, 1)) {
            Some(_v) => &'#',
            None => &'.'
        }
    }
}
#[derive(Clone)]
struct Image {
    tiles: Vec<Tile>
}
impl Image {
    fn from_string(s: Vec<String>) -> Image {
        let mut tiles: Vec<Tile> = Vec::new();
        let start_re = Regex::new(r#"Tile (?P<id>[0-9]+):"#).unwrap();
        for (i,line) in s.clone().into_iter().enumerate() {
            if start_re.is_match(&line) { tiles.push(Tile::from_string(s[i..].to_vec())); }
        }
        Image{ tiles: tiles }
    }
    fn solve(&mut self) -> Option<Image> {
        let permutation_iter = self.tiles.clone().into_iter().permutations(self.tiles.len());
        for permutation in permutation_iter {
            let mut candidate = Image { tiles: permutation };
            if candidate.align_tiles((0,0)) { return Some(candidate); }
            println!("\x1B[1;1H\n{:#?}", candidate);
        }
        None
    }
    fn align_tiles(&mut self, (x,y): (i32, i32)) -> bool {
        let image_dimension: i32 = (self.tiles.len() as f64).sqrt() as i32;
        if x == image_dimension { return self.align_tiles((0,y+1)); }
        if y == image_dimension { return true; }
        for transform in self.tiles[0].transformations() {
            self[(x,y)].set_transformation(transform);
            if (x == 0 || self[(x-1,y)].right() == self[(x,y)].left()) &&
               (y == 0 || self[(x,y-1)].bottom() == self[(x,y)].top()) &&
               self.align_tiles((x+1, y)) { 
                return true;
               }
        }
        return false;
    }
}
impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let image_dimension: i32 = (self.tiles.len() as f64).sqrt() as i32;
        let tile_dimension: Point3::<i32> = self.tiles.clone().into_iter().next().unwrap().dimension;
        (0..image_dimension).for_each(|imagey| {
            (0..tile_dimension.y).for_each(|tiley| {
                (0..image_dimension).for_each(|imagex| {
                    (0..tile_dimension.x).for_each(|tilex| {
                        write!(f, "{}", self[(imagex, imagey)][(tilex, tiley)]).unwrap();
                    });
                    write!(f, " ").unwrap();
                });
                writeln!(f, "").unwrap();
            });
            writeln!(f, "").unwrap();
        });
        Ok(())
    }
}
impl Index<(i32,i32)> for Image {
    type Output = Tile;

    fn index(&self, coord: (i32, i32) ) -> &Self::Output {
        &self.tiles[coord.1 as usize * ((self.tiles.len() as f64).sqrt() as usize) + coord.0 as usize]
    }
}
impl IndexMut<(i32,i32)> for Image {

    fn index_mut(&mut self, coord: (i32, i32) ) -> &mut Self::Output {
        let len = (self.tiles.len() as f64).sqrt() as usize;
        &mut self.tiles[coord.1 as usize * len + coord.0 as usize]
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Image = Image::from_string(std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect());
    let solution = input.solve().unwrap();
    println!("{:#?}", solution );
    println!("{:#?}", solution[(1,0)] );
    println!("{:#?}", solution[(1,0)].transformation );
    println!("{:#?}", solution[(1,0)].left() );
    Ok(())
}