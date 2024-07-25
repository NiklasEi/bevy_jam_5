use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Player,
    Ground,
}
