#[macro_use]
extern crate bencher;
use ::matrix::*;

benchmark_group!(
    matrix,
    matrix::benchers::normal_050,
    // matrix::benchers::mpsc2_050,
    // matrix::benchers::mpsc2_100,
    // matrix::benchers::mpsc2_200,
    // matrix::benchers::mpsc2_300,
    // matrix::benchers::mpsc4_050,
    // matrix::benchers::mpsc4_100,
    // matrix::benchers::mpsc4_200,
    // matrix::benchers::mpsc4_300,
);

benchmark_main!(matrix);

// matrix::benchers::mpsc2_050,
// matrix::benchers::mpsc2_100,
// matrix::benchers::mpsc2_200,
// matrix::benchers::mpsc2_300,
// matrix::benchers::mpsc2_400,
// matrix::benchers::mpsc2_500,
// matrix::benchers::mpsc4_050,
// matrix::benchers::mpsc4_100,
// matrix::benchers::mpsc4_200,
// matrix::benchers::mpsc4_300,
// matrix::benchers::mpsc4_400,
// matrix::benchers::mpsc4_500,
//
// matrix::benchers::mpsc2_600,
// matrix::benchers::mpsc2_700,
// matrix::benchers::mpsc2_800,
// matrix::benchers::mpsc2_900,
// matrix::benchers::mpsc4_600,
// matrix::benchers::mpsc4_700,
// matrix::benchers::mpsc4_800,
// matrix::benchers::mpsc4_900,
