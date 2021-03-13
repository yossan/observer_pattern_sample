use super::subject::NumberGenerator;

pub trait Observer: PartialEq {
    fn update<T: Observer, O: NumberGenerator<T> + ?Sized>(&self, generator: &O);
}

#[derive(PartialEq)]
pub struct DegitObserver;

impl Observer for DegitObserver {
    fn update<T: Observer, O: NumberGenerator<T> + ?Sized>(&self, generator: &O) {
        println!("DegitObserver: {}", generator.get_number());
    }
}

#[derive(PartialEq)]
pub struct GraphObserver;

impl Observer for GraphObserver {
    fn update<T: Observer, O: NumberGenerator<T> + ?Sized>(&self, generator: &O) {
        print!("GraphObserver:");
        (0..generator.get_number()).for_each(|_| { print!("*") });
        println!("");
    }

}
