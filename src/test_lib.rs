#[cfg(test)]
use super::Bitset;
#[test]
fn test_shift_up1() {
    use typenum::consts::*;

    let mut set = Bitset::<U100>::new();
    for i in 0..100 {
        set.insert(i);
    }
    set.shift_up();
    assert!(!set.contains(0));
    for i in 1..100 {
        assert!(set.contains(i));
    }
}

#[test]
fn test_shift_up2() {
    use typenum::consts::*;

    let mut set = Bitset::<U100>::new();
    set.insert(0);
    set.insert(3);
    set.insert(7);
    set.insert(16);
    set.insert(99);
    set.shift_up();
    for i in 0..100 {
        let elems: Vec<usize> = vec![1, 4, 8, 17];
        
        assert!(set.contains(i) == elems.contains(&i));
    }
}

#[test]
fn test_shift_up_odd_to_even() {
    use typenum::consts::*;

    let mut set = Bitset::<U100>::new();
    for i in 1..51 {
        set.insert(2 * i - 1);
    }
    set.shift_up();
    for i in 0..100 {
        assert!(set.contains(i) == (i % 2 == 0 && i > 0));
    }
}
#[test]
fn test_shift_up_even_to_odd() {
    use typenum::consts::*;

    let mut set = Bitset::<U100>::new();
    for i in 0..50 {
        set.insert(2 * i);
    }
    set.shift_up();
    for i in 0..100 {
        assert!(set.contains(i) == (i % 2 != 0));
    }
}

#[test]
fn test_shift_down1() {
    use typenum::consts::*;

    let mut set = Bitset::<U100>::new();
    for i in 0..100 {
        set.insert(i);
    }
    set.shift_down();
    assert!(!set.contains(99));
    for i in 0..99 {
        assert!(set.contains(i));
    }
}

#[test]
fn test_shift_down2() {
    use typenum::consts::*;

    let mut set = Bitset::<U100>::new();
    set.insert(0);
    set.insert(3);
    set.insert(7);
    set.insert(16);
    set.insert(99);
    set.shift_down();
    for i in 0..100 {
        let elems: Vec<usize> = vec![2, 6, 15, 98];
        assert!(set.contains(i) == elems.contains(&i));
    }
}

#[test]
fn test_shift_down_odd_to_even() {
    use typenum::consts::*;

    let mut set = Bitset::<U100>::new();
    for i in 1..51 {
        set.insert(2 * i - 1);
    }
    set.shift_down();
    for i in 0..100 {
        assert!(set.contains(i) == (i % 2 == 0));
    }
}
#[test]
fn test_shift_down_even_to_odd() {
    use typenum::consts::*;

    let mut set = Bitset::<U100>::new();
    for i in 0..50 {
        set.insert(2 * i);
    }
    set.shift_down();
    for i in 0..100 {
        assert!(set.contains(i) == (i % 2 != 0 && i < 99));
    }
}
