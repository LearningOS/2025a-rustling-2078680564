// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

mod sausage_factory {
    // 保持私有，不让模块外访问
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 标记为 pub，让模块外可以访问
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
