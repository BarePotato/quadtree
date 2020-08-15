use rand::Rng;
use sfml::{
    graphics::{
        CircleShape, Color, RectangleShape, RenderStates, RenderTarget, RenderWindow, Shape,
        Transformable,
    },
    system::Vector2f,
};

const WIN_W: u32 = 800;
const WIN_H: u32 = 600;

type Rect = sfml::graphics::Rect<f32>;

#[derive(Debug, Default, Clone)]
struct Quadtree {
    bounds: Rect,
    capacity: usize,
    children: Vec<Vector2f>,
    quads: Option<Vec<Quadtree>>,
}

// Core impl for Quadtree is here, other stuff is below in a 2nd impl for everything else
impl Quadtree {
    fn new() -> Quadtree {
        Quadtree {
            bounds: Rect::new(0.0, 0.0, WIN_W as f32, WIN_H as f32),
            capacity: 4,
            children: Vec::with_capacity(4),
            quads: None,
        }
    }

    fn insert(&mut self, location: Vector2f) -> bool {
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

        self.quads = Some(vec![Quadtree::new(); 4]);

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

    fn _query(&self, range: Rect) -> Vec<Vector2f> {
        let mut children_in_range = Vec::new();

        if self.bounds.intersection(&range).is_none() {
            return children_in_range;
        }

        for child in self.children.iter() {
            if range.contains(*child) {
                children_in_range.push(*child);
            }
        }

        if self.quads.is_none() {
            return children_in_range;
        }

        for quad in self.quads.as_ref().unwrap().iter() {
            children_in_range.append(&mut quad._query(range));
        }

        children_in_range
    }
}

// Render and utility functions for Quadtree, Core stuff is above.
impl Quadtree {
    fn draw(&self, window: &RenderWindow) {
        let mut my_rect = RectangleShape::with_size(
            (self.bounds.width - 2 as f32, self.bounds.height - 2 as f32).into(),
        );
        my_rect.set_position((self.bounds.left + 1 as f32, self.bounds.top + 1 as f32));
        my_rect.set_fill_color(Color::TRANSPARENT);
        my_rect.set_outline_color(Color::RED);
        my_rect.set_outline_thickness(1.0);
        window.draw_rectangle_shape(&my_rect, RenderStates::default());

        if self.quads.is_some() {
            for quad in self.quads.as_ref().unwrap().iter() {
                quad.draw(window);
            }
        }

        if self.children.len() > 0 {
            let mut my_dot = CircleShape::new(1.0, 8);
            my_dot.set_fill_color(Color::GREEN);

            for child in &self.children {
                my_dot.set_position(*child);
                window.draw_circle_shape(&my_dot, RenderStates::default());
            }
        }
    }
}

fn main() {
    let mut random = rand::thread_rng();

    let mut quad_root = Quadtree::new();

    let mut vectors = Vec::new();
    for _idx in 0..10000 {
        let rng_x = random.gen_range(0, WIN_W) as f32;
        let rng_y = random.gen_range(0, WIN_H) as f32;
        let my_vector = Vector2f::new(rng_x, rng_y);
        vectors.push(my_vector);
    }

    for vector in vectors {
        quad_root.insert(vector);
    }

    // dbg!(&quad_root);
    // dbg!(&quad_root.quads.as_ref().unwrap()[0].children.capacity());
    // dbg!(&quad_root.quads.as_ref().unwrap()[0].quads.as_ref().unwrap().capacity());

    // fixme ------------------------------------------
    // Setup
    let mut window = RenderWindow::new(
        (WIN_W, WIN_H),
        "Quadtree",
        sfml::window::Style::CLOSE,
        &Default::default(),
    );

    // Loop
    while window.is_open() {
        // Input
        while let Some(event) = window.poll_event() {
            use sfml::window::{Event, Key};

            match event {
                Event::Closed => window.close(),
                Event::MouseButtonPressed { .. } => {}
                Event::KeyPressed { code, .. } => {
                    if code == Key::Escape {
                        window.close();
                    }
                }
                _ => {}
            }
        }

        // Update
        {}

        // Render
        {
            window.clear(Color::BLACK);

            // Draw
            {
                quad_root.draw(&window);
            }

            window.display();
        }

        // std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_nanos(1));
    }
}
