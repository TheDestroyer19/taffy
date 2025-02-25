pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(60f32),
                    height: taffy::style::Dimension::Points(60f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: taffy::style::AlignItems::Center,
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(100f32), ..Default::default() },
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
