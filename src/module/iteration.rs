/*
Calculate u_ij with Finite Difference Methods (FDM)
Boundary Condition: D = [0, 1] * [0, 1] (100 * 100 mesh)
*/
// Input
const NX: usize = 100; // x-direction splitted number
const NY: usize = 100; // y-direction splitted number

pub fn iteration(mut u: [[f64; NX]; NY]) -> [[f64; NX]; NY] {
    // Initialize
    let mut u_next: [[f64; NX]; NY] = [[0.0; NX]; NY];

    // Calculate u_ij
    for i in 1..NX - 1 {
        for j in 1..NY - 1 {
            u_next[i][j] = (u[i][j - 1] + u[i - 1][j] + u[i + 1][j] + u[i][j + 1]) / 4.0;
        }
    }

    // Update u_ij
    for i in 1..NX - 1 {
        for j in 1..NY - 1 {
            u[i][j] = u_next[i][j];
        }
    }
    return u;
}
// End of the function iteration()
