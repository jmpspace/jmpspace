
// Delete all of this eventually

use std::f64::consts::PI;
use ship::{Attach, Beam, Part, Structure};
use tagtree;

// TODO can further simplify with the builder macros

pub fn simple_ship() -> Structure {

    let (forward, reverse, left, right) = (0,1,2,3);
    let attitude_attach = Attach { location: 10.0, rotation: 0.0 };

    let engine_l = box part!(engine!(4.0, 4.0, left));
    let attitude_l = beam!(10.0, vec!((attitude_attach.clone(), engine_l)));

    let engine_r = box part!(engine!(4.0, 4.0, right));
    let attitude_r = beam!(10.0, vec!((attitude_attach, engine_r)));

    let engine_rev = box part!(engine!(4.0, 4.0, reverse));
    let rev_l_attach = Attach { location: 10.0, rotation: -PI };
    let rev_l = beam!(10.0, vec!((rev_l_attach, engine_rev)));

    let engine_rev = box part!(engine!(4.0, 4.0, reverse));
    let rev_r_attach = Attach { location: 10.0, rotation: PI };
    let rev_r = beam!(10.0, vec!((rev_r_attach, engine_rev)));

    let mods = vec!(
        attach_tree!(6.0, 3.0 * PI / 2.0, rev_l),
        attach_tree!(6.0, PI / 2.0, rev_r),
        attach_tree!(42.0, 3.0 * PI / 2.0, attitude_l),
        attach_tree!(42.0, PI / 2.0, attitude_r),
        attach_part!(0.0, 0.0, vessel!(7.0, 7.0)),
        attach_part!(25.0, 0.0, fuel_tank!(14.0, 30.0)),
        attach_part!(50.0, 0.0, engine!(13.0, 13.0, forward)),
        );

    beam!(50.0, mods)
}
