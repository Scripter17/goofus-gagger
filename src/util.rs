pub fn is_default<T: Default + Eq>(x: &T) -> bool {x == &T::default()}
