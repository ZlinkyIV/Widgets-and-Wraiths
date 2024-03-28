#[derive(Clone, Copy, Debug)]
pub enum Position {
    Intangible,
    Tangible(i32, i32),
}

impl Position {
    pub fn distance_to(&self, other: &Position) -> DistanceTo {
        match (self, other) {
            (
                Position::Tangible(ax, ay),
                Position::Tangible(bx, by),
            ) => DistanceTo::SquareDistance(calc_square_dist((*ax, *ay), (*bx, *by))),
            _ => DistanceTo::Intangible,
        }
    }

    // pub fn square_distance_between(&self, other: &Position) -> Option<u32> {
    //     match (self, other) {
    //         (
    //             Position::Tangible(self_pos_x, self_pos_y), 
    //             Position::Tangible(other_pos_x, other_pos_y)
    //         ) => Some({
    //             let change_x = (self_pos_x - other_pos_x).abs() as u32;
    //             let change_y = (self_pos_y - other_pos_y).abs() as u32;
                
    //             change_x * change_x + change_y * change_y
    //         }),
    //         _ => None,
    //     }
    // }

    // pub fn distance_between(&self, other: &Position) -> Option<f64> {
    //     Some((self.distance_between(other)? as f64).sqrt())
    // }
}

fn calc_square_dist((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> u32 {
    let change_x = (ax - bx).abs() as u32;
    let change_y = (ay - by).abs() as u32;

    change_x * change_x + change_y * change_y
}

#[derive(PartialEq)]
pub enum DistanceTo {
    Intangible,
    SquareDistance(u32),
}

impl DistanceTo {
    pub fn square_dist(&self) -> Option<u32> {
        match self {
            Self::SquareDistance(square_dist) => Some(*square_dist),
            Self::Intangible => None,
        }
    }

    pub fn dist(&self) -> Option<f64> {
        Some((self.square_dist()? as f64).sqrt())
    }
}

impl PartialOrd for DistanceTo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (
                Self::SquareDistance(dist_a),
                Self::SquareDistance(dist_b),
            ) => Some(dist_a.cmp(dist_b)),
            _ => None,
        }
    }
}

impl From<u32> for DistanceTo {
    fn from(value: u32) -> Self {
        DistanceTo::SquareDistance(value * value)
    }
}

// impl Ord for DistanceTo {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        
//     }
// }

// impl From<DistanceTo> for u32 {
//     fn from(value: DistanceTo) -> Self {
//         value.square_distance
//     }
// }

// impl DistanceTo {
//     fn dist(self) -> f64 {
//         (self.square_distance as f64).sqrt()
//     }
// }