use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn sum(a: u32, b: u32) -> u32 {
  a + b
}

#[node_bindgen]
fn hello_plus_world(hello: String) -> String {
  hello.to_owned() + " world"
}
