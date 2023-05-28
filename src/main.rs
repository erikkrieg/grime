mod transpile;

fn main() {
    let out = transpile::transpile("Transpile world!");
    println!("{out}");
}
