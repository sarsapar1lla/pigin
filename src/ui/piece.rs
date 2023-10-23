use ratatui::{
    style::Color,
    widgets::canvas::{self, Line, Shape},
};

pub struct Pawn {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub colour: Color,
}

impl Shape for Pawn {
    fn draw(&self, painter: &mut canvas::Painter) {
        let base_height = self.y + (self.width * 0.2);
        let head_height = self.y + (self.width * 0.6);
        let head_width = self.width * 0.3;

        let head_left_x = self.x + (self.width - head_width) * 0.5;
        let head_right_x = (self.x + self.width) - (self.width - head_width) * 0.5;
        let lines = [
            Line {
                x1: self.x,
                y1: self.y,
                x2: self.x + self.width,
                y2: self.y,
                color: self.colour,
            },
            Line {
                x1: self.x,
                y1: self.y,
                x2: self.x,
                y2: base_height,
                color: self.colour,
            },
            Line {
                x1: self.x + self.width,
                y1: self.y,
                x2: self.x + self.width,
                y2: base_height,
                color: self.colour,
            },
            Line {
                x1: self.x,
                y1: base_height,
                x2: self.x + self.width,
                y2: base_height,
                color: self.colour,
            },
            Line {
                x1: self.x,
                y1: base_height,
                x2: head_left_x,
                y2: head_height,
                color: self.colour,
            },
            Line {
                x1: self.x + self.width,
                y1: base_height,
                x2: head_right_x,
                y2: head_height,
                color: self.colour,
            },
            Line {
                x1: head_left_x,
                y1: head_height,
                x2: head_right_x,
                y2: head_height,
                color: self.colour,
            },
            Line {
                x1: head_left_x,
                y1: head_height,
                x2: head_left_x,
                y2: self.y + self.width,
                color: self.colour,
            },
            Line {
                x1: head_right_x,
                y1: head_height,
                x2: head_right_x,
                y2: self.y + self.width,
                color: self.colour,
            },
            Line {
                x1: head_left_x,
                y1: self.y + self.width,
                x2: head_right_x,
                y2: self.y + self.width,
                color: self.colour,
            },
        ];
        for line in &lines {
            line.draw(painter);
        }
    }
}

pub struct Rook {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub colour: Color,
}

impl Shape for Rook {
    fn draw(&self, painter: &mut canvas::Painter) {
        let neck_width = self.width * 0.7;
        let head_height = self.y + (self.width * 0.6);

        let neck_left_x = self.x + (self.width - neck_width) * 0.5;
        let neck_right_x = (self.x + self.width) - (self.width - neck_width) * 0.5;
        let lines = [
            Line {
                x1: neck_left_x,
                y1: self.y,
                x2: neck_right_x,
                y2: self.y,
                color: self.colour,
            },
            Line {
                x1: neck_left_x,
                y1: self.y,
                x2: neck_left_x,
                y2: head_height,
                color: self.colour,
            },
            Line {
                x1: neck_right_x,
                y1: self.y,
                x2: neck_right_x,
                y2: head_height,
                color: self.colour,
            },
            Line {
                x1: self.x,
                y1: head_height,
                x2: self.x + self.width,
                y2: head_height,
                color: self.colour,
            },
            Line {
                x1: self.x,
                y1: head_height,
                x2: self.x,
                y2: self.y + self.width,
                color: self.colour,
            },
            Line {
                x1: self.x + self.width,
                y1: head_height,
                x2: self.x + self.width,
                y2: self.y + self.width,
                color: self.colour,
            },
            Line {
                x1: self.x,
                y1: self.y + self.width,
                x2: self.x + (self.width * 0.1),
                y2: self.y + self.width,
                color: self.colour,
            },
            Line {
                x1: self.x + (self.width * 0.9),
                y1: self.y + self.width,
                x2: self.x + self.width,
                y2: self.y + self.width,
                color: self.colour,
            },
            Line {
                x1: self.x + (self.width * 0.3),
                y1: self.y + self.width,
                x2: self.x + (self.width * 0.7),
                y2: self.y + self.width,
                color: self.colour,
            },
            Line {
                x1: self.x + (self.width * 0.3),
                y1: head_height + (self.width * 0.3),
                x2: self.x + (self.width * 0.3),
                y2: self.y + self.width,
                color: self.colour,
            },
            Line {
                x1: self.x + (self.width * 0.7),
                y1: head_height + (self.width * 0.3),
                x2: self.x + (self.width * 0.7),
                y2: self.y + self.width,
                color: self.colour,
            },
        ];
        for line in &lines {
            line.draw(painter);
        }
    }
}
