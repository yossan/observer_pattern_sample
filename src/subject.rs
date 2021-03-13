use super::observer::Observer;

pub trait NumberGenerator<O: Observer> {
    fn observers(&self) -> &Vec<O>;
    fn mut_observers(&mut self) -> &mut Vec<O>;

    fn add_observer(&mut self, observer: O) {
        let observers = self.mut_observers();
        observers.push(observer);
    }

    fn remove_observer(&mut self, observer: &O) {
        let observers = self.mut_observers();
        if let Some(pos) = observers.iter().position(|x| x == observer) {
            observers.remove(pos);
        }
    }

    fn notify_observers(&self) {
        let observers = self.observers();
        observers.iter().for_each(|observer| observer.update(self));
    }

    fn get_number(&self) -> i32;
    fn execute(&mut self);
}

pub struct RandomNumberGenerator<T> {
    number: i32,
    _observers: Vec<T>,
}

impl<T> RandomNumberGenerator<T> 
    where
        T: Observer,
{
    pub fn new() -> Self {
        RandomNumberGenerator {
            number: 0,
            _observers: vec![],
        }
    }
}


impl<T> NumberGenerator<T> for RandomNumberGenerator<T>
where
    T: Observer,
{

    fn observers(&self) -> &Vec<T> {
        &self._observers
    }

    fn mut_observers(&mut self) -> &mut Vec<T> {
        &mut self._observers
    }

    fn get_number(&self) -> i32 {
        self.number
    }

    fn execute(&mut self) {
        for i in 0..20 {
            self.number = i;
            self.notify_observers();
        }
    }
}

