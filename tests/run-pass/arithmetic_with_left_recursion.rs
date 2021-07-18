extern crate peg;

use arithmetic::sum;

peg::parser!( grammar arithmetic() for str {
    #[cache]
    pub rule sum() -> i64
        = l:sum() "+" r:number() { l+r }
        / number()

    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }
});

fn main() {
    assert_eq!(sum("1+1"), Ok(2));
    assert_eq!(sum("1+1+1"), Ok(3));
}