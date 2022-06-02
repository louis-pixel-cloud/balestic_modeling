pub mod weapon;
pub mod vec;

const g : f32 = 9.806;

#[allow(dead_code)]
fn getScope(angle : i32 , weapon : Weapon) -> f32{
    return ((weapon.initial_speed * weapon.initial_speed)*(2*angle).sin())/g;
}

#[allow(dead_code)]
fn getTop(angle : i32 , weapon : Weapon) -> vec{
    return vec{
        x : getScope(angle , weapon)/2,
        y : (weapon.initial_speed * weapon.initial_speed)*angle.sin()*angle.sin()/(2*g),
    };
}