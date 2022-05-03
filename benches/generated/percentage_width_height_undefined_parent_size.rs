pub fn compute() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Percent(0.5f32),
                    height: stretch2::style::Dimension::Percent(0.5f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style { flex_direction: stretch2::style::FlexDirection::Column, ..Default::default() },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
}