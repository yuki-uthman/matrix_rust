use super::*;
use ::bencher::black_box;
use ::bencher::Bencher;

pub fn normal_050(bench: &mut Bencher) {
    bench.iter(|| normal::calculate(50));
    bench.iter(|| normal::calculate(60));
    bench.iter(|| normal::calculate(70));
}

pub fn normal_100(bench: &mut Bencher) {
    bench.iter(|| normal::calculate(100));
}

pub fn normal_200(bench: &mut Bencher) {
    bench.iter(|| normal::calculate(200));
}

pub fn normal_300(bench: &mut Bencher) {
    bench.iter(|| normal::calculate(300));
}

pub fn normal_400(bench: &mut Bencher) {
    bench.iter(|| normal::calculate(400));
}

pub fn normal_500(bench: &mut Bencher) {
    bench.iter(|| normal::calculate(500));
}

pub fn normal_rotation_050(bench: &mut Bencher) {
    bench.iter(|| normal::calculate_with_rotation(50));
}

pub fn normal_rotation_100(bench: &mut Bencher) {
    bench.iter(|| normal::calculate_with_rotation(100));
}

pub fn normal_rotation_200(bench: &mut Bencher) {
    bench.iter(|| normal::calculate_with_rotation(200));
}

pub fn normal_rotation_300(bench: &mut Bencher) {
    bench.iter(|| normal::calculate_with_rotation(300));
}

pub fn normal_rotation_400(bench: &mut Bencher) {
    bench.iter(|| normal::calculate_with_rotation(400));
}

pub fn normal_rotation_500(bench: &mut Bencher) {
    bench.iter(|| normal::calculate_with_rotation(500));
}

pub fn mpsc2_050(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 50));
}

pub fn mpsc2_100(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 100));
}

pub fn mpsc2_200(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 200));
}

pub fn mpsc2_300(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 300));
}

pub fn mpsc2_400(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 400));
}

pub fn mpsc2_500(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 500));
}

pub fn mpsc2_600(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 600));
}

pub fn mpsc2_700(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 700));
}

pub fn mpsc2_800(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 800));
}

pub fn mpsc2_900(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 900));
}

pub fn mpsc2_1000(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(2, 1000));
}

pub fn mpsc4_050(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 50));
}

pub fn mpsc4_100(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 100));
}

pub fn mpsc4_200(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 200));
}

pub fn mpsc4_300(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 300));
}

pub fn mpsc4_400(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 400));
}

pub fn mpsc4_500(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 500));
}

pub fn mpsc4_600(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 600));
}

pub fn mpsc4_700(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 700));
}

pub fn mpsc4_800(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 800));
}

pub fn mpsc4_900(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 900));
}

pub fn mpsc4_1000(bench: &mut Bencher) {
    bench.iter(|| with_mpsc2::calculate(4, 1000));
}

pub fn crossbeam(bench: &mut Bencher) {
    // bench.iter(|| with_crossbeam::calculate(1_000_000));
}

// pub fn more_optimized(bench: &mut Bencher) {
//     bench.iter(|| implementation4::answer());
// }
