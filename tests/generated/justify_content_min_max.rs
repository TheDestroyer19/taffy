#[test]
fn justify_content_min_max() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(60f32),
                    height: stretch2::style::Dimension::Points(60f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                justify_content: stretch2::style::JustifyContent::Center,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                min_size: stretch2::geometry::Size {
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: stretch2::geometry::Size {
                    height: stretch2::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 60f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 60f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 20f32);
}