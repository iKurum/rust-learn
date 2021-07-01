mod lib;
use lib::lcp7;

fn main() {
    let relation = vec![vec![0,2],vec![2,1],vec![3,4],vec![2,3],vec![1,4],vec![2,0],vec![0,4]];
    let r = lcp7::Solution::num_ways(5, relation, 3);
    println!("{}", r);
}
