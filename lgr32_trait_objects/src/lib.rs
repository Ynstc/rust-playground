/* 32.0.0 Trait objects */
// more flexibility but has performance cost

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

//why not use generic instead of trait object case: (wrong)
// look there is T which will set one type of data,
// so you cannot store mix of data (e.g buttons input fields etc.)
// but homogenous type (only buttons or only input fields etc.)
// if only one type then this solution with generics are preferable because of runtime cost

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
//     where T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // draw button
    }
}

//'object safe' traits rules:
// all methods on that trait:
// 1. return type is not 'self'
// 2. are no generic parameters
