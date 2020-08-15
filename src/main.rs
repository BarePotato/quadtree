use sfml::graphics::{
    Color, Rect, RectangleShape, RenderStates, RenderTarget, RenderWindow, Shape, Transformable,
};
use sfml::system::Vector2f;





const QUAD_CAPACITY: usize = 4; // Amount of children a quad can store


#[derive(Debug, Default, Clone)]
struct Quadtree {
    bounds: Rect<f32>,
    is_div: bool,
    capacity: usize,
    children: Vec<Vector2f>,
    quads: Vec<Quadtree>,
}

impl Quadtree {
    fn insert(&mut self, location: Vector2f) -> bool {
        if !self.bounds.contains(location) {
            return false;
        }
    
        if self.children.len() < self.capacity && !self.is_div {
            self.children.push(location);
            return true;
        }

        if !self.is_div {
            self.divide();
        }

        for quad in self.quads.iter_mut() {
            if quad.insert(location) {
                return true;
            }
        }

        false
    }

    fn divide(&mut self) {
        let x = self.bounds.left;
        let y = self.bounds.top;
        let w = self.bounds.width / 2.0;
        let h = self.bounds.height / 2.0;

        for (idx, quad) in self.quads.iter_mut().enumerate() {
            match idx {
                0 => quad.bounds = Rect::new(x, y, w, h), // NW
                1 => quad.bounds = Rect::new(w, y, w, h), // NE
                2 => quad.bounds = Rect::new(w, h, w, h), // SE
                3 => quad.bounds = Rect::new(x, h, w, h), // SW
                _ => { panic!("More quads than quarters!"); }
            }
        }

        self.is_div = true;
    }

    fn query(&self, range: Rect<f32>) -> Vec<Vector2f> {
        let mut children_in_range = Vec::new();

        if self.bounds.intersection(&range).is_none() {
            return children_in_range;
        }

        for child in self.children.iter(){
            if range.contains(*child) {
                children_in_range.push(*child);
            }
        }

        if !self.is_div {
            return children_in_range;
        }

        for quad in self.quads.iter() {
            children_in_range.append(&mut quad.query(range));
        }

        children_in_range
    }
}

fn main() {
    let mut quad_root = Quadtree {
        bounds: Rect::new(0.0, 0.0, 800.0, 600.0),
        is_div: false,
        capacity: QUAD_CAPACITY,
        children: vec![Vector2f::default(); QUAD_CAPACITY],
        quads: vec![Quadtree::default(); 4],
    };
    // quad_root.divide();

    dbg!(quad_root);





let my_vec: Vec<Quadtree> = Vec::with_capacity(4);
















    // Setup
    let mut window = RenderWindow::new(
        (800, 600),
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
                let mut my_rect = RectangleShape::with_size(Vector2f::new(100.0, 100.0));
                my_rect.set_position((10.0, 10.0));
                my_rect.set_fill_color(Color::TRANSPARENT);
                my_rect.set_outline_color(Color::RED);
                my_rect.set_outline_thickness(1.0);
                window.draw_rectangle_shape(&my_rect, RenderStates::default());
            }

            window.display();
        }

        std::thread::yield_now();
    }
}
