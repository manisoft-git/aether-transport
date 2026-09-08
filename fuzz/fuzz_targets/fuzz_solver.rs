#![no_main]
use libfuzzer_sys::fuzz_target;
use aether_math::matrix::GaussianSolver;

fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return;
    }
    const K: usize = 4;
    const SYMBOL_SIZE: usize = 8;
    let mut solver = GaussianSolver::new(K, SYMBOL_SIZE);

    let mut offset = 0;
    while offset + K + SYMBOL_SIZE <= data.len() {
        let coeffs = &data[offset..offset + K];
        let payload = &data[offset + K..offset + K + SYMBOL_SIZE];
        let _ = solver.add_row(coeffs, payload);
        offset += K + SYMBOL_SIZE;
        if solver.is_solved() {
            let _ = solver.reconstructed();
            break;
        }
    }
});
