

struct CustomMaterial {
    time: f32;
};

[[group(1), binding(0)]]
var<uniform> material: CustomMaterial;

[[stage(fragmeent)]]
fn fragment(
    [[builting(position)]]: coord: vec4<f32>,    
)
