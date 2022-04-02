wit_bindgen_rust::import!("../../host.wit");

fn main() {
    host::print(&format!("Doubled number {} is {}", 10, host::double(10.)));
}    