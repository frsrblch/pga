use super::*;

#[test]
fn add_blades() {
    let a = E1(1.0);
    let b = E1(2.0);

    assert_eq!(E1(3.0), a + b);
}

#[test]
fn sub_blades() {
    let a = E1(1.0);
    let b = E1(2.0);

    assert_eq!(E1(-1.0), a - b);
}

#[test]
fn add_grades() {
    let a = Vector {
        e0: E0(1.0),
        e1: E1(2.0),
        e2: E2(3.0),
        e3: E3(5.0),
    };

    let expected = Vector {
        e0: E0(2.0),
        e1: E1(4.0),
        e2: E2(6.0),
        e3: E3(10.0),
    };

    assert_eq!(expected, a + a);
}
