#[test]
fn max_width_overrides_width_on_root() {
    let mut stretch = stretch2::Stretch::new();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                max_size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
}