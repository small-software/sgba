mod sgba;
mod test;

fn main() {
    sgba::api::psgba();
    sgba::api::auth::jwt::p();
    sgba::api::data::users::controller::test_controller_mod();
    println!("Hello, world!");
}
