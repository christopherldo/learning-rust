mod binding_and_mutability;
mod scope;
mod shadowing;

fn main() {
    binding_and_mutability::run();
    scope::run();
    shadowing::run();
}
