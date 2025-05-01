mod gcd;
mod node_list;

use crate::gcd::Gcd;
use crate::node_list::NodeList;

fn main() {
    let mut list = NodeList::new();

    list.insert(1);
    list.insert(2);

    list.remove(1);

    assert_eq!(list.search(1), false);

    list.print();

    let ans = 10.gcd(15);
    println!("gcd(10, 15) = {}", ans);
    // assert_eq!(ans == 5, true);
}
