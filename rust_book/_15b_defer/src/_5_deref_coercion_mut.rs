///# HOW DEREF COERCION INTERACTS WITH MUTABILITY
/// - DerefMut trait to override the * operator on mutable references

///# RUST DOES DEREF COERCION WHEN:
/// - It finds types and trait implementations in three cases:
/// - From &T to &U when T: Deref<Target=U>
/// - From &mut T to &mut U when T: DerefMut<Target=U>
/// - From &mut T to &U when T: Deref<Target=U>
