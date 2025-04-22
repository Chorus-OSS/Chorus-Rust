use shipyard::Component;

#[derive(Component)]
pub struct Ageable {
    age: i32,
}

impl Ageable {
    pub fn default() -> Self {
        Self { age: 0 }
    }
}
