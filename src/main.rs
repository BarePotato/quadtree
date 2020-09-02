use sfml::{
    graphics::{
        CircleShape, Color, RectangleShape, RenderStates, RenderTarget, RenderWindow, Shape,
        Transformable,
    },
    system::Vector2f,
};

use quadtree::{self, Quadtree};

//----- Settings ------//
const WIN_W: f32 = 800.0;
const WIN_H: f32 = 600.0;
const MAX_CAP: usize = 2;

trait Drawable {
    fn draw(&self, window: &mut RenderWindow);
}

impl Drawable for Quadtree {
    fn draw(&self, window: &mut RenderWindow) {
        if self.quads.is_none() && self.capacity <= 0 {
            return;
        }

        if self.quads.is_some() {
            for quad in self.quads.as_ref().unwrap().iter() {
                quad.draw(window);
            }
        }

        if self.children.len() > 0 {
            let random = &mut rand::thread_rng();

            use rand::Rng;
            let my_color = Color::rgb(
                random.gen_range(u8::MAX / 8, u8::MAX),
                random.gen_range(u8::MAX / 8, u8::MAX),
                random.gen_range(u8::MAX / 8, u8::MAX),
            );

            let mut my_rect = RectangleShape::with_size(
                (self.bounds.width - 2.0, self.bounds.height - 2.0).into(),
            );
            my_rect.set_position((self.bounds.left + 1.0, self.bounds.top + 1.0));
            my_rect.set_fill_color(Color::TRANSPARENT);
            my_rect.set_outline_color(my_color);
            my_rect.set_outline_thickness(1.0);
            window.draw(&my_rect);

            let mut my_dot = CircleShape::new(2.0, 8);
            my_dot.set_fill_color(my_color);

            for child in &self.children {
                my_dot.set_position(Vector2f::from((child.x, child.y)));
                window.draw_circle_shape(&my_dot, RenderStates::default());
            }
        }
    }
}

fn main() {
    let bounds = quadtree::Rect::new(0.0, 0.0, WIN_W, WIN_H);
    let mut quad_root = Quadtree::new(bounds, MAX_CAP).set_capacity(0).set_quads();

    // Setup
    let window = &mut RenderWindow::new(
        (WIN_W as u32, WIN_H as u32),
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
        {
            quad_root.clear();

            // Random points for testing.
            use rand_distr::{Distribution, Normal};
            let normal_w = Normal::new(WIN_W as f32 / 2.0, WIN_W as f32 / 8.0).unwrap();
            let normal_h = Normal::new(WIN_H as f32 / 2.0, WIN_H as f32 / 8.0).unwrap();
            let random = &mut rand::thread_rng();
            let mut vectors = Vec::new();
            for _idx in 0..500 {
                let rng_x = normal_w.sample(random) as f32;
                let rng_y = normal_h.sample(random) as f32;
                let my_vector = quadtree::Vector2f::new(rng_x, rng_y);
                vectors.push(my_vector);
            }

            for vector in vectors {
                quad_root.insert(vector);
            }
        }

        // Render
        {
            window.clear(Color::BLACK);

            // Draw
            {
                quad_root.draw(window);
            }

            window.display();
        }

        // std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_millis(1000 / 1));
    }
}
