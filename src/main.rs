use project_euler_rust::problems::SOLVERS;

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .expect("problem number")
        .parse()
        .unwrap();

    println!("{}", SOLVERS[n - 1]());
}
