use super::archive_decode::DisplayDecoder;
use crate::Result;

pub(super) fn visual(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    for label in ["color", "wire_color", "hlr_color"] {
        archive.u32(label)?;
    }
    archive.scope("material", material)?;
    archive.text("addin_identifier")?;
    archive.i32s(&["valid_only_for_addin", "use_in_sw_lw_rendering"])?;
    archive.text("appearance_name")?;
    archive.i32s(&[
        "hardcoded_facet_display",
        "default_appearance",
        "unknown_78",
        "unknown_7c",
        "graphic_mesh_color_property",
    ])?;
    if archive.i32("has_dspbr")? != 0 {
        archive.scope("dspbr", super::modern_dspbr::read)?;
    }
    Ok(())
}

fn material(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("user_name")?;
    archive.text("id_name")?;
    archive.f64s(&[
        "ambient",
        "diffuse",
        "specular",
        "shininess",
        "transparency",
        "emission",
    ])?;
    if archive.i32("has_texture")? != 0 {
        archive.scope("texture", texture)?;
    }
    material_rendering(archive)
}

fn material_rendering(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&[
        "use_advanced_graphics",
        "use_defined_texture",
        "use_defined_shader",
        "use_defined_color",
        "use_defined_hatch",
    ])?;
    archive.text("classification")?;
    archive.i32("use_new_shaders")?;
    if archive.i32("has_rv_app_props")? != 0 {
        archive.scope("rv_app_props", rv)?;
    }
    archive.text("plm_id")?;
    if archive.i32("has_texture2")? != 0 {
        archive.scope("texture2", texture)?;
    }
    archive.scope("user_property", super::modern_lights::user_properties)
}

fn texture(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.f64s(&[
        "unknown_8",
        "unknown_10",
        "texture_ranges[0]",
        "texture_ranges[1]",
        "unknown_28",
        "unknown_30",
    ])?;
    for index in 0..5 {
        archive.f64(&format!("texture_dir[{index}]"))?;
    }
    archive.f64s(&["unknown_60", "texture_angle"])?;
    archive.text("unknown_70")?;
    archive.i16("texture_render_mode")?;
    archive.i32("ignore_color_material")?;
    archive.f64s(&["texture_trans_x", "texture_trans_y"])?;
    archive.i32("for_c_thread")?;
    Ok(())
}

fn rv(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.u32("unknown_0")?;
    archive.u32("unknown_4")?;
    archive.text("unknown_8")?;
    archive.u8("texture_mapping_mode")?;
    archive.u32("material_color2")?;
    for prefix in ["u_dir", "v_dir", "center"] {
        float3(archive, prefix)?;
    }
    archive.f32("face_width")?;
    archive.i32("color_form")?;
    archive.f32s(&[
        "reflectivity",
        "color_mix",
        "orig_ambient",
        "orig_diffuse",
        "orig_specular",
    ])?;
    rv_illumination(archive)
}

fn rv_illumination(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("illum_shader_type")?;
    archive.f32s(&["axis_direction1", "axis_direction2"])?;
    archive.text("unknown_68")?;
    archive.u32("specular_color")?;
    archive.f32s(&["roughness", "index_of_refraction"])?;
    archive.text("unknown_80")?;
    archive.f32s(&[
        "bump_strength",
        "displacement_distance",
        "luminous_intensity",
    ])?;
    archive.i32("flags")?;
    rv_tail(archive)
}

fn rv_tail(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.f32("refraction_roughness")?;
    archive.i32("surface_finish_shader_type")?;
    float3(archive, "appearance_u_dir")?;
    float3(archive, "appearance_v_dir")?;
    archive.f32s(&[
        "appearance_trans_x",
        "appearance_trans_y",
        "knurl_height_cap",
        "knurl_angle",
        "dimple_size",
        "default_width",
        "default_height",
    ])?;
    archive.text("lxp_filename")?;
    archive.f32("round_sharp_edges")
}

pub(super) fn float3(archive: &mut DisplayDecoder<'_>, prefix: &str) -> Result<()> {
    for index in 0..3 {
        archive.f32(&format!("{prefix}[{index}]"))?;
    }
    Ok(())
}
