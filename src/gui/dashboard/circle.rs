//! This module defines the circle component for circular layouts.

use dioxus::prelude::*;

/// Represents a point in 2D space with x and y coordinates.
#[derive(Clone, PartialEq)]
pub struct Point {
    /// The x coordinate of the point.
    pub x: f32,

    /// The y coordinate of the point.
    pub y: f32,
}

/// Properties for the `Circle` component, which arranges child elements in a circular layout.
#[derive(Props, Clone, PartialEq)]
pub struct CircleSlotProps {
    /// The total number of slots in the circle.
    total_slots: usize,

    /// The radius of the circle in pixels.
    radius: f32,

    /// The child elements to be arranged in the circle.
    slots: Vec<Element>,

    /// The size of each (quadratic) slot in pixels.
    slot_size: f32,

    /// The center point of the circle
    center: Point,

    /// The rotation of the circle in slots.
    ///
    /// A `0.5` rotation means that each slot is rotated by half a slot.
    rotation: f32,
}

/// The `Circle` component for arringing elements in a circular layout.
#[component]
pub fn Circle(props: CircleSlotProps) -> Element {
    rsx! {
        div {
            {props.slots.iter().enumerate().map(|(i, slot)| {
                let angle_deg = ((i as f32 + props.rotation) / props.total_slots as f32) * 360.0; // Angle in degrees
                let angle_rad = angle_deg.to_radians(); // Convert to radians for sin/cos
                let x = props.center.x + props.radius * angle_rad.cos() - props.slot_size / 2.0; // X offset
                let y = props.center.y + props.radius * angle_rad.sin() - props.slot_size / 2.0; // Y offset

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
