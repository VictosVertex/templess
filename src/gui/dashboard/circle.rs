use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Props, Clone, PartialEq)]
pub struct CircleSlotProps {
    total_slots: usize,
    radius: f32, // Distance from the center (in pixels)
    slots: Vec<Element>,
    slot_size: f32,
    center: Point,
    rotation: f32,
}

#[component]
pub fn Circle(props: CircleSlotProps) -> Element {
    rsx! {
        div {
            {props.slots.iter().enumerate().map(|(i, slot)| {
                let angle_deg = ((i as f32 + props.rotation) / props.total_slots as f32) * 360.0; // Angle in degrees
                let angle_rad = angle_deg.to_radians(); // Convert to radians for sin/cos
                let x = props.center.x + props.radius * angle_rad.cos() - (props.slot_size as f32) / 2.0; // X offset
                let y = props.center.y + props.radius * angle_rad.sin() - (props.slot_size as f32) / 2.0; // Y offset

                rsx! {
                    div {
                        class: "absolute flex items-center justify-center transition-all duration-200",
                        style: format!(
                            "width: {}px; height: {}px; transform: translate({}px, {}px);",
                            props.slot_size, props.slot_size, x, y
                        ),
                        {slot}
                    }
                }
            })}
        }
    }
}
