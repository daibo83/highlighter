use highlighter::{get_highlight_offsets, highlight};
use std::time::Instant;
pub fn main() {
    let needle = ["linh", "nguyen"];
    let haystack = "linh nguyen the sdf sdiufh sadofij";
    let now = Instant::now();
    let res = highlight(&needle, haystack, "<marked>", "</marked>");
    println!("{:?}, {:?}", res, now.elapsed());
    let now = Instant::now();
    let res = get_highlight_offsets(&needle, haystack);
    println!("{:?}, {:?}", res, now.elapsed());
}