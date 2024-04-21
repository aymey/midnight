use generational_arena::{Arena, Index};

#[derive(Default)]
pub struct Scene {
    store: Arena<Shape>,
    shapes: Vec<ShapeIndex>
}

impl Scene {
    pub fn append(&mut self, shape: Shape) -> ShapeIndex {
        let index = self.store.insert(shape);
        let si = ShapeIndex::new(index);
        self.shapes.push(si);

        si
    }

    pub fn draw(&mut self) {
        for (_, shape) in self.store.iter() {
            println!("{:?}", shape.position);
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    x: f32,
    y: f32
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Default, Copy, Clone)]
pub enum ShapeKind {
    #[default]
    Square,
    Circle
}

#[derive(Default, Copy, Clone)]
pub struct Shape {
    kind: ShapeKind,
    position: Point,
    size: u32,
    rotation: u32,
    index: Option<ShapeIndex>
}

impl Shape {
    pub fn new(index: Index) -> Self {
        let mut shape = Shape::default();
        shape.index = Some(ShapeIndex::new(index));

        shape
    }

    pub fn move_by(&mut self, offset: Point) {
        self.position += offset;
    }
}

#[derive(Copy, Clone)]
pub struct ShapeIndex {
    pub inner: Index,
    pub parent: Option<Index>,
    pub child: Option<Index>,
}

impl ShapeIndex {
    pub fn new(index: Index) -> Self {
        Self {
            inner: index,
            parent: None,
            child: None
        }
    }

    pub fn set_child(&mut self, child: &mut ShapeIndex) {
        self.child = Some(child.inner);
        child.parent = Some(self.inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_move_by() { }
}
