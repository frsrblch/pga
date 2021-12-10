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

#[test]
fn sub_grades() {
    let a = Vector {
        e0: E0(1.0),
        e1: E1(2.0),
        e2: E2(3.0),
        e3: E3(5.0),
    };

    let b = Vector {
        e0: E0(2.0),
        e1: E1(4.0),
        e2: E2(6.0),
        e3: E3(10.0),
    };

    assert_eq!(a, b - a);
}

#[test]
fn neg_vector() {
    let a = Vector {
        e0: E0(1.0),
        e1: E1(2.0),
        e2: E2(3.0),
        e3: E3(5.0),
    };

    let expected = Vector {
        e0: E0(-1.0),
        e1: E1(-2.0),
        e2: E2(-3.0),
        e3: E3(-5.0),
    };

    assert_eq!(expected, -a);
}

#[test]
fn combine_grades() {
    let weight = VectorWeight { e0: E0(1.0) };
    let bulk = VectorBulk {
        e1: E1(2.0),
        e2: E2(3.0),
        e3: E3(5.0),
    };
    let expected = Vector {
        e0: E0(1.0),
        e1: E1(2.0),
        e2: E2(3.0),
        e3: E3(5.0),
    };

    assert_eq!(expected, weight + bulk);
}

#[test]
fn mul_assign_grade() {
    let mut a = Vector {
        e0: E0(1.0),
        e1: E1(2.0),
        e2: E2(3.0),
        e3: E3(5.0),
    };

    a *= 2.0;

    let expected = Vector {
        e0: E0(2.0),
        e1: E1(4.0),
        e2: E2(6.0),
        e3: E3(10.0),
    };

    assert_eq!(expected, a);
}

#[test]
fn div_assign_grade() {
    let mut a = Vector {
        e0: E0(2.0),
        e1: E1(4.0),
        e2: E2(6.0),
        e3: E3(10.0),
    };

    a /= 2.0;

    let expected = Vector {
        e0: E0(1.0),
        e1: E1(2.0),
        e2: E2(3.0),
        e3: E3(5.0),
    };

    assert_eq!(expected, a);
}

#[test]
fn mul_blades() {
    assert_eq!(E12(1.0), E1(1.0) * E2(1.0));
    assert_eq!(E2(-1.0), E12(1.0) * E1(1.0));
    assert_eq!(-1.0, E12(1.0) * E12(1.0));
}

#[test]
fn geo_f64_e1() {
    assert_eq!(E1(6.0), 2f64.geo(E1(3.0)));
}
