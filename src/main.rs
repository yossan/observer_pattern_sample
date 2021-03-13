mod subject;
mod observer;

use subject::*;
use observer::*;

fn main() {
    let degitobserver = DegitObserver;
    let graphobserver = GraphObserver;
    let mut generator = RandomNumberGenerator::new();
    //generator.add_observer(degitobserver);
    generator.add_observer(graphobserver);
    generator.execute();
}
