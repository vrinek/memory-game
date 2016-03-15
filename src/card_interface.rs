use piston_window::*;

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

    pub fn render<G>(&self, c: Context, g: &mut G)
        where G: Graphics
    {
        let red = [1.0, 0.0, 0.0, 1.0];
        let yellow = [1.0, 1.0, 0.0, 1.0];
        let green = [0.0, 1.0, 0.0, 1.0];
        let blue = [0.0, 0.5, 1.0, 1.0];

        let color = match self.over {
            true => yellow,
            false => red,
        };
        let color = match self.selected {
            true => green,
            false => color,
        };
        let color = match self.scored {
            true => blue,
            false => color,
        };

        rectangle(color, self.origin_and_size(), c.transform, g);
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
