mod binding_and_mutability;
mod destructuring;
mod destructuring_assignments;
mod scope;
mod shadowing;
mod unused_variables;

fn main() {
    binding_and_mutability::run();
    scope::run();
    shadowing::run();
    unused_variables::run();
    destructuring::run();
    destructuring_assignments::run();
}
