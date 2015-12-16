use cgmath::Array;
use super::entity::Entity;
use super::entity::EntityState;
use super::entity::Kind;

pub fn find_collisions(state: &EntityState,
                       entities: &[Entity])
                       -> Vec<((u32, Kind), (u32, Kind))> {
    let mut collisions: Vec<((u32, Kind), (u32, Kind))> = Vec::new();
    let collidables = entities.iter()
                              .map(|e| {
                                  (e.id,
                                   *state.positions.get(&e.id).unwrap(),
                                   state.scales.get(&e.id).unwrap().max() / 2.0)
                              })
                              .collect::<Vec<_>>();

    for &(a, a_position, a_radius) in &collidables {
        for &(b, b_position, b_radius) in &collidables {
            let d = b_position - a_position;
            let distance = d.x.powf(2.0) + d.y.powf(2.0);
            if distance < (a_radius + b_radius).powf(2.0) {
                let kind_a = state.kinds.get(&a).unwrap();
                let kind_b = state.kinds.get(&b).unwrap();
                collisions.push(((a, (*kind_a).clone()), (b, (*kind_b).clone())));
            }
        }
    }
    collisions
}
