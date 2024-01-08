pub mod linkedlist;
pub mod tree;
pub mod graphe;

use linkedlist::LinkedList;

use crate::tree::BinaryTree;
use graphe::SimpleGraphe;
use graphe::DirectedWeightedGraphe;

fn test_linked_list() {
    let mut s = LinkedList::new(Some(3));

    println!("{}", s);
    s.append(4);
    s.append(5);
    println!("{}", s);
    println!("Length of list is {}", s.len());
    let result = s.pop();
    println!("Result of pop is: {}", result);
    println!("{}", s);
    println!("Result of pop is: {}", s.pop());
    println!("Length of list is {}", s.len());
    println!("{}", s);
}

fn test_trees() {
    // Test with trees
    let mut t: tree::BinaryTree<i32> = BinaryTree::new();

    println!("-=-=-=-=-=-=-=-=-=-=-");
    println!("{}", t);
    t.append(6);
    t.append(5);
    t.append(4);
    t.append(3);
    println!("{}", t);

    let bt = t.rebalance();
    println!("{}", bt);
}

fn test_graphs() {
    let connections: Vec<(char, char)> = vec![
        ('1', '2'),
        ('1', '3'),
        ('2', '5'),
        ('3', '4'),
        ('3', '5'),
        ('4', '5')
    ];
    let mut g: SimpleGraphe<char> = SimpleGraphe::new(5, &connections, true);
    println!("Graph display:");
    g.display();
    g.depth_first_print('1');
    g.breadth_first_print('1');

    let distances: Vec<(&str, &str, i32)> = vec![
        ("Tokyo", "Yokohama", 27),
        ("Osaka", "Nagoya", 139),
        ("Kyoto", "Osaka", 43),
        ("Nagoya", "Yokohama", 250),
        ("Nagoya", "Nagano", 250),
        ("Nagano", "Kanazawa", 139),
        ("Tokyo", "Sendai", 302),
        ("Sendai", "Aomori", 286),
        ("Osaka", "Hiroshima", 280),
        ("Sapporo", "Kimobetsu",66),
        ("Kimobetsu", "Kutchan", 25),
        ("Kutchan", "Yoichi", 41),
        ("Yoichi", "Otaru", 20),
        ("Otaru", "Sapporo", 39),
        ("Kutchan", "Oshamambe", 78),
        ("Oshamambe", "Kimobetsu", 76),

    ];

    let japan_roads: DirectedWeightedGraphe<&str> = DirectedWeightedGraphe::new(10, &distances, true);
    japan_roads.display();

    japan_roads.simple_shortest_path(&"Nagano", &"Hiroshima");

    println!("-=-=-=-=-= Djikstra -=-=-=-=-=-=");
    let min_distances_1 = japan_roads.djikstra_shortest_path(&"Kutchan");

    for (town, distance) in min_distances_1 {
        println!("Distance from {} to {} is {}","Kutchan",town, distance);
    }


}

fn main() {
    test_graphs();
}
