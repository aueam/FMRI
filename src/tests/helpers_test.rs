use crate::helpers::check_character_collision;

#[test]
fn _check_character_collision() {
    check_character_collision(&"qwertyuiopasdfghjklzxcvbnm1234567890".to_owned()).unwrap();
    check_character_collision(&"test,".to_owned()).unwrap();
    check_character_collision(&"test-".to_owned()).unwrap();
    check_character_collision(&"test:".to_owned()).unwrap();
}

#[test]
#[should_panic]
fn _check_character_collision_panic1() {
    check_character_collision(&"test@".to_owned()).unwrap();
}
