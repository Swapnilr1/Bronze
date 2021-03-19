use std::rc::Rc;
use std::cell::RefCell;


use crate::cookbook::*;
use crate::genetics::*;


#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}



impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {r, g, b}
    }

    pub fn cross(c1: &Color, c2: &Color) -> Color {
        Color {
            r: cross8(c1.r, c2.r),
            g: cross8(c1.g, c2.g),
            b: cross8(c1.b, c2.b),
        }
    }
}

#[derive(Debug)]
pub struct Turtle {
    walking_speed: u32,
    color: Color,
    favorite_flavor: Flavor,

    // Can't just have a vector of children because the children are owned by the campus and they can't have two owners.
    children: Vec<Rc<RefCell<Turtle>>>,
}


impl Turtle {
    // You normally don't create a new Turtle from nothing; instead, breed
    // two Turtles.
    // Lifetimes are critical and tricky here!
    // The output lifetime doesn't depend on the lifetimes
    // of the parameters.
    pub fn breed(p1: Rc<RefCell<Turtle>>, p2: Rc<RefCell<Turtle>>) -> Turtle {
        let t1 = p1.borrow();
        let t2 = p2.borrow();

        Turtle {
            walking_speed: cross32(t1.walking_speed, t2.walking_speed),
            color: Color::cross(&t1.color, &t2.color),
            favorite_flavor: Flavor::random_flavor(),
            children: Vec::new(),
        }
    }

    pub fn walking_speed(&self) -> u32 {
        self.walking_speed
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn favorite_flavor(&self) -> Flavor {
        self.favorite_flavor
    }

    pub fn new(walking_speed: u32, favorite_flavor: Flavor, color: Color) -> Turtle {
        Turtle {
            walking_speed, color, favorite_flavor, children: vec![]
        }
    }

    pub fn choose_recipe<'a>(&self, cookbook: &'a Cookbook) -> Option<&'a Recipe> {
        for recipe in cookbook.recipes() {
            if recipe.flavor() == self.favorite_flavor() {
                return Some(recipe);
            }
        }
        None
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Turtle>>) {
        self.children.push(child);
    }
}
