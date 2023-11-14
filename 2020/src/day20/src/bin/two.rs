/*use std::{io::BufRead, collections::{HashSet, HashMap}, ops::Mul};
use itertools::{Itertools};

use nalgebra::{matrix, Matrix3, vector};
use regex::Regex;
#[derive(Clone,Debug,Hash,PartialEq,Eq)]
struct Coord {
    x: i64,
    y: i64
}
#[derive(Clone)]
struct Tile {
    id: i64,
    dimension: Coord,
    on: HashSet<Coord>
}
impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile {}:", self.id).unwrap();
        (0..self.dimension.y).for_each(|y| {
            (0..self.dimension.x).for_each(|x| { write!(f, "{}", if self.on.contains(&Coord { x:x, y:y }) { "#" } else {"."} ).unwrap(); });
            writeln!(f, "").unwrap();
        });
        Ok(())
    }
}
impl Mul<Matrix3<i64>> for Tile {
    type Output = Self;
    fn mul(self, rhs: Matrix3<i64>) -> Self {
        Tile {
            id: self.id.clone(),
            dimension: self.dimension.clone(),
            on: self.on.into_iter()
                .map(|c| rhs * vector![c.x, c.y, 1])
                .map(|v| Coord { x: v[0] / v[2], y: v[1] / v[2]})
                .fold(HashSet::new(), |mut hs, c| { hs.insert(c); hs })
        }
    }
}
impl Tile {
    fn from_string(s: Vec<String>) -> Tile {
        let start_re = Regex::new(r#"Tile (?P<id>[0-9]+):"#).unwrap();
        Tile {
            id: start_re.captures(&s[0]).unwrap().name("id").unwrap().as_str().parse::<i64>().unwrap(),
            dimension: Coord { x: s[1].len() as i64, y: s[1].len() as i64 },
            on: s[1..s[1].len() + 1]
                .into_iter()
                .enumerate()
                .map(move |(y,line)| line
                    .chars()
                    .into_iter()
                    .enumerate()
                    .map(move |(x, ch)| if ch == '#' { Some(Coord { x:x.clone() as i64, y:y.clone() as i64 }) } else { None })
                    .filter(|c| c.is_some() )
                )
                .flatten()
                .fold(HashSet::new(), |mut hs, c| { hs.insert(c.unwrap()); hs })

        }
    }
    fn matches_right(&self, other: Tile) -> bool {
        (0..self.dimension.y)
            .map(|y| if self.on.contains(&Coord{ x: self.dimension.x-1, y: y}) { "#" } else { "." })
            .zip( (0..other.dimension.y).map(|y| if other.on.contains(&Coord{ x: 0, y: y}) { "#" } else { "." }) )
            .map(|pair| pair.0 == pair.1)
            .reduce(|acc,t| acc && t)
            .unwrap()
    }
    fn matches_down(&self, other: Tile) -> bool {
        (0..self.dimension.x)
            .map(|x| if self.on.contains(&Coord{ x: x, y: self.dimension.y-1 }) { "#" } else { "." })
            .zip( (0..other.dimension.x).map(|x| if other.on.contains(&Coord{ x: x, y: 0 }) { "#" } else { "." }) )
            .map(|pair| pair.0 == pair.1)
            .reduce(|acc,t| acc && t)
            .unwrap()
    }
    fn transformations(&self) -> Vec<Matrix3<i64>> {
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
            .collect_vec()
    }
}
#[derive(Clone)]
struct Tiles(Vec<Tile>);
type TilesIter = dyn Iterator<Item=Box<Tiles>>;
impl Tiles {
    fn new() -> Tiles { Tiles { 0: Vec::new() }}
    fn transformations(&self) -> TilesIter {
        match self.0.into_iter().next() {
            Some(tile) => 
                Tiles{ 0: self.0[1..].to_vec() }
                    .transformations()
                    .into_iter()
                    .cartesian_product( tile
                        .transformations()
                        .into_iter()
                        .map( |transformation| tile.clone() * transformation )
                    )
                    .map( |(sub_result, local_result)| { let mut tiles = [ local_result ].to_vec(); tiles.append(&mut sub_result.0.clone()); Tiles { 0: tiles } } )
                ,
            None => panic!("Unexpected")
        }
    }
}
#[derive(Clone)]
struct Image {
    tiles: HashMap<Coord, Tile>
}
impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let image_dimension: i64 = (self.tiles.len() as f64).sqrt() as i64;
        let tile_dimension: Coord = self.tiles.clone().into_iter().next().unwrap().1.dimension;
        (0..image_dimension).for_each(|imagey| {
            (0..tile_dimension.y).for_each(|tiley| {
                (0..image_dimension).for_each(|imagex| {
                    let tile = self.tiles[& Coord{ x: imagex, y: imagey }].clone();
                    (0..tile_dimension.x).for_each(|tilex| {
                        write!(f, "{}", if tile.on.contains( &Coord{ x: tilex, y: tiley }) { "#" } else { "." }).unwrap();
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
impl Image {
    fn from_string(s: Vec<String>) -> Image {
        let mut tiles: Tiles = Tiles::new();
        let start_re = Regex::new(r#"Tile (?P<id>[0-9]+):"#).unwrap();
        for (i,line) in s.clone().into_iter().enumerate() {
            if start_re.is_match(&line) { tiles.0.push(Tile::from_string(s[i..].to_vec())); }
        }
        Image::from_tiles(tiles)
    }
    fn from_tiles(tiles: Tiles) -> Image {
        let dimension: i64 = (tiles.0.len() as f64).sqrt() as i64;
        Image {
            tiles: tiles.0.into_iter().enumerate().fold( HashMap::new(), |mut hm, (index, tile)| { hm.insert( Coord{ x: index as i64 % dimension, y: index as i64 / dimension }, tile); hm } )
        }
    }
    /*
    fn from_tile_permutations(tiles: Tiles) -> Vec<Image> {
        let len = tiles.0.len();
        tiles.0.into_iter()
            .permutations(len)
            .map(|permutation| Image::from_tiles(permutation))
            .collect()
    }
    fn from_string_permutations(s: Vec<String>) -> Vec<Image> {
        let mut tiles: Tiles = Tiles::new();
        let start_re = Regex::new(r#"Tile (?P<id>[0-9]+):"#).unwrap();
        for (i,line) in s.clone().into_iter().enumerate() {
            if start_re.is_match(&line) { tiles.0.push(Tile::from_string(s[i..].to_vec())); }
        }
        Image::from_tile_permutations(tiles)
    }
    */
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Image = Image::from_string(std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect());
//    println!("{:#?}", input);
//    println!("{:#?}", [1,2,3].into_iter().permutations(3).collect::<Vec<Vec<i64>>>());
    println!("{:#?}", input.tiles[&Coord{ x:0, y: 0 }].transformations().len());
//    println!("{:#?}", input.tiles[&Coord{ x:0, y: 0 }].clone() * input.tiles[&Coord{ x:0, y: 0 }].transformations()[5]);
    Ok(())
}
*/
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}