use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn sum(a: u32, b: u32) -> u32 {
  a + b
}
