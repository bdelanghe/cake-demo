use bakery::{frosting::frost, slice::cut, sponge::Sponge, Bake};

#[test]
fn sponge_bakes_longer_with_more_cocoa() {
    assert!(Sponge { cocoa_g: 200 }.minutes() > Sponge { cocoa_g: 50 }.minutes());
}

#[test]
fn slices_add_up_to_the_whole_cake() {
    let cake = frost(Sponge { cocoa_g: 120 });
    let total: f64 = cut(&cake, 8).iter().map(|s| s.share).sum();
    assert!((total - 1.0).abs() < 1e-9);
}
