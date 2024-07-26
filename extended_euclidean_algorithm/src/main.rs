use gcd::Gcd;

fn main() {
    let a: i128 = 13;
    let b: i128 = 12;
    sanitize_input_ab(a, b);
    let (x, y, z) = extended_euclidean_algorithm(a, b);
    assert_euclidean_proof(a, b, y, z);
    println!("The Extended Euclidean GCD is: {}", x);
}

fn sanitize_input_ab(a: i128, b: i128) {
    if a <= 0 || b <= 0 || a < b {
        panic!("input integer a{a} & b{b} cannot be ZERO")
    }
}

fn assert_euclidean_proof(a: i128, b: i128, y: i128, z: i128) {
    let m = a as u128;
    let n = b as u128;
    let gcd_result = m.gcd(n);
    let coefficient_addition = (y * a) + (z * b);
    let i_gcd_result = gcd_result as i128;
    assert_eq!(i_gcd_result, coefficient_addition);
}
// fn division (a: i128, b: i128) -> i128 {
//     let result = a /b;
//     println!("result is: {}", result);
//     return result;

// }
fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128) {
    let mut r0 = a;
    let mut r1 = b;
    let mut s0: i128 = 1;
    let mut s1: i128 = 0;
    let mut t0: i128 = 0;
    let mut t1: i128 = 1;
    // let mut rk_2;
    // let mut sk_2;
    // let mut tk_2;
    // let mut rk_1;
    // let mut sk_1: i128;
    // let mut tk_1;
    let mut rk;
    let mut sk;
    let mut tk: i128;
    // starting the loop and break if remainder = 0
    // let mut qk: i128;
    //  //updating the quoficient
    //  qk = r0 / r1 ;
    //  //updating the remainder
    //  rk = r0 - (qk * r1);
    //updating the coefficient s and t
    //  rk_1 = rk;

    rk = b;
    while rk != 0 {
        let qk = r0 / r1;
        //updating the remainder
        rk = r0 - (qk * r1);
        sk = s0 - (qk * s1);
        tk = t0 - (qk * t1);
        // rk_1 = rk;
        //updating the quoficient

        //updating the coefficient s and t
        //updating the variables
        r0 = r1;
        r1 = rk;
        s0 = s1;
        s1 = sk;
        t0 = t1;
        t1 = tk;
    }
    return (r0, s0, t0);
}

#[test]
#[should_panic]
fn sanitize_works() {
    sanitize_input_ab(45, 0);
}
#[test]
fn extended_euclidean_algorithm_works() {
    let (a, b, c) = extended_euclidean_algorithm(45, 10);
    let expected_a = 5;
    let expected_b = 1;
    let expected_c = -4;

    assert_eq!(a, expected_a);
    assert_eq!(b, expected_b);
    assert_eq!(c, expected_c);
}

// #[test]
// fn division_works () {
//     assert_eq!(4, division(45, 10));
// }
