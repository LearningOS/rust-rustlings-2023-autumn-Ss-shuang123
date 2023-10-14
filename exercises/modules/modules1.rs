/*
 * @Author: ss
 * @Date: 2023-10-13 09:53:14
 * @LastEditTime: 2023-10-13 09:53:14
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\modules\modules1.rs
 */
// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.
 

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        // get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
