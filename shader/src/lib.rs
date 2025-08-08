#![cfg_attr(target_arch = "spirv", no_std)]

mod fragment;
mod vertex;
use spirv_std::glam::Vec4;
use spirv_std::spirv;

#[spirv(fragment)]
pub fn main_fs(output: &mut Vec4) {
    fragment::main(output);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: usize,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
    out_color: &mut Vec4,
) {
    vertex::main(vert_id, out_pos, out_color);
}
