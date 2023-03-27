use enum2repr::EnumRepr;

#[derive(EnumRepr, Debug, PartialEq, Copy, Clone)]
#[repr(u16)]
enum Color {
    Red = 0x04,
    Green = 0x15,
    Blue = 0x34,
}

#[test]
fn convert_variants() {
    assert_eq!(Ok(Color::Red), Color::try_from(0x04));
    assert_eq!(Ok(Color::Green), Color::try_from(0x15));
    assert_eq!(Ok(Color::Blue), Color::try_from(0x34));
}

#[test]
fn convert_variants_back() {
    assert_eq!(u16::from(Color::Red), 0x04);
    assert_eq!(u16::from(Color::Green), 0x15);
    assert_eq!(u16::from(Color::Blue), 0x34);
}
