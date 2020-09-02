#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }
}

pub type Vector2f = Vector2<f32>;

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T> {
    pub fn new(left: T, top: T, width: T, height: T) -> Self {
        Rect {
            left,
            top,
            width,
            height,
        }
    }

    pub fn from_points(position: Vector2<T>, size: Vector2<T>) -> Self {
        Rect {
            left: position.x,
            top: position.y,
            width: size.x,
            height: size.y,
        }
    }
}

impl<T: PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy> Rect<T> {
    pub fn contains(self, position: Vector2<T>) -> bool {
        let x = position.x;
        let y = position.y;
        let (min_x, max_x) = (
            min(self.left, self.left + self.width),
            max(self.left, self.left + self.width),
        );
        let (min_y, max_y) = (
            min(self.top, self.top + self.height),
            max(self.top, self.top + self.height),
        );

        // <= ?
        x >= min_x && x < max_x && y >= min_y && y < max_y
    }
}

fn min<T: PartialOrd>(i: T, n: T) -> T {
    if i < n {
        i
    } else {
        n
    }
}

fn max<T: PartialOrd>(i: T, n: T) -> T {
    if i > n {
        i
    } else {
        n
    }
}

#[derive(Debug, Default, Clone)]
pub struct Quadtree {
    pub bounds: Rect<f32>,
    pub capacity: usize,
    max_capacity: usize,
    pub children: Vec<Vector2f>,
    pub quads: Option<Vec<Quadtree>>,
}

impl Quadtree {
    pub fn new(bounds: Rect<f32>, capacity: usize) -> Self {
        Quadtree {
            bounds,
            capacity,
            max_capacity: capacity,
            children: Vec::with_capacity(capacity),
            quads: None,
        }
    }

    pub fn set_bounds(mut self, bounds: Rect<f32>) -> Self {
        self.bounds = bounds;

        self
    }

    pub fn set_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self.children = Vec::with_capacity(capacity);

        self
    }

    pub fn set_quads(mut self) -> Self {
        self.quads = Some(vec![Quadtree::new(self.bounds, self.capacity); 4]);

        self
    }

    pub fn insert(&mut self, location: Vector2f) -> bool {
        if !self.bounds.contains(location) {
            return false;
        }

        if self.children.len() < self.capacity && self.quads.is_none() {
            self.children.push(location);
            return true;
        }

        if self.quads.is_none() {
            self.divide();
        }

        for quad in self.quads.as_mut().unwrap().iter_mut() {
            if quad.insert(location) {
                return true;
            }
        }

        false
    }

    fn divide(&mut self) {
        if self.quads.is_some() {
            return;
        }

        let x = self.bounds.left;
        let y = self.bounds.top;
        let w = self.bounds.width / 2.0;
        let h = self.bounds.height / 2.0;

        self.quads = Some(vec![Quadtree::new(self.bounds, self.max_capacity); 4]);

        // todo: better way to do this?
        for (idx, quad) in self.quads.as_mut().unwrap().iter_mut().enumerate() {
            match idx {
                0 => quad.bounds = Rect::new(x, y, w, h),         // NW
                1 => quad.bounds = Rect::new(x + w, y, w, h),     // NE
                2 => quad.bounds = Rect::new(x + w, y + h, w, h), // SE
                3 => quad.bounds = Rect::new(x, y + h, w, h),     // SW
                _ => {
                    panic!("More quads than quarters!");
                }
            }
        }
    }

    // todo: this wasn't in use yet, so... quiet
    // fn _query(&self, range: Rect<f32>) -> Vec<Vector2f> {
    //     let mut children_in_range = Vec::new();

    //     if self.bounds.intersection(&range).is_none() {
    //         return children_in_range;
    //     }

    //     for child in self.children.iter() {
    //         if range.contains(*child) {
    //             children_in_range.push(*child);
    //         }
    //     }

    //     if self.quads.is_none() {
    //         return children_in_range;
    //     }

    //     for quad in self.quads.as_ref().unwrap().iter() {
    //         children_in_range.append(&mut quad._query(range));
    //     }

    //     children_in_range
    // }

    fn _remove_nearest(&mut self, _location: Vector2f) {}

    pub fn clear(&mut self) {
        self.children.clear();
        self.quads = None;
    }
}