use piston_window::*;
use opengl_graphics::glyph_cache::GlyphCache;

pub struct CardInterface {
    pub index: usize,
    origin: [f64; 2],
    size: [f64; 2],
    pub selected: bool,
    pub scored: bool,
    pub over: bool,
}

impl CardInterface {
    pub fn new(index: usize) -> CardInterface {
        let x_offset: f64 = 150.0 * (((index - 1) / 2) as f64);
        let y_offset: f64 = 150.0 * ((index % 2) as f64);

        CardInterface {
            index: index,
            origin: [50.0 + x_offset, 50.0 + y_offset],
            size: [100.0, 100.0],
            selected: false,
            scored: false,
            over: false,
        }
    }

    pub fn render<G>(&self, context: Context, graphics: &mut G, cache: &mut GlyphCache<'static>)
        where G: Graphics
    {
        let red = [1.0, 0.0, 0.0, 1.0];
        let yellow = [1.0, 1.0, 0.0, 1.0];
        let green = [0.0, 1.0, 0.0, 1.0];
        let blue = [0.0, 0.5, 1.0, 1.0];
        let black = [0.0, 0.0, 0.0, 1.0];

        let color = if self.scored {
            blue
        } else if self.selected {
            green
        } else if self.over {
            yellow
        } else {
            red
        };

        rectangle(color, self.origin_and_size(), context.transform, graphics);
        let mut text = Text::new(14);
        text.color = black;
        text.draw(&"GAME OVER",
                  cache,
                  &context.draw_state,
                  context.transform,
                  graphics);

        // TODO: render card's number
    }

    pub fn bounds(&self) -> [f64; 4] {
        [self.origin[0],
         self.origin[1],
         self.origin[0] + self.size[0],
         self.origin[1] + self.size[1]]
    }

    fn origin_and_size(&self) -> [f64; 4] {
        [self.origin[0], self.origin[1], self.size[0], self.size[1]]
    }
}
