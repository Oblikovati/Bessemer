use super::MaterialRead;
use crate::Result;

pub(crate) fn visual(archive: &mut impl MaterialRead) -> Result<()> {
    for (threshold, label) in [(0x90, "color"), (0xd3, "wire_color"), (0xac, "hlr_color")] {
        if archive.version() > threshold {
            archive.u32(label)?;
        }
    }
    archive.scope("material", material)?;
    visual_addin(archive)?;
    if archive.version() > 0xb53 {
        archive.i32("use_in_sw_lw_rendering")?;
    }
    if archive.version() > 0xb55 {
        archive.text("appearance_name")?;
    }
    visual_tail(archive)
}

fn visual_addin(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() <= 0xb2b {
        return Ok(());
    }
    archive.text("addin_identifier")?;
    archive.i32("valid_only_for_addin").map(|_| ())
}

fn visual_tail(archive: &mut impl MaterialRead) -> Result<()> {
    visual_appearance_state(archive)?;
    if archive.version() > 0x144b {
        archive.i32("unknown_78")?;
    }
    if archive.version() > 0x2419 {
        archive.i32("unknown_7c")?;
    }
    if archive.version() > 0x2794 {
        archive.i32("graphic_mesh_color_property")?;
    }
    if archive.version() > 0x3f08 && archive.i32("has_dspbr")? != 0 {
        archive.scope("dspbr", super::dspbr::read)?;
    }
    Ok(())
}

fn visual_appearance_state(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() > 0xdb1 {
        archive.i32("hardcoded_facet_display")?;
    }
    if archive.version() < 0xe55 {
        return Ok(());
    }
    let appearance = if archive.version() < 0x1052 {
        "unknown_old_5c"
    } else {
        "default_appearance"
    };
    archive.i32(appearance).map(|_| ())
}

pub(crate) fn material(archive: &mut impl MaterialRead) -> Result<()> {
    archive.text("user_name")?;
    archive.text("id_name")?;
    archive.f64s(&["ambient", "diffuse", "specular", "shininess"])?;
    if archive.version() > 0x172 {
        archive.f64s(&["transparency", "emission"])?;
    }
    if archive.version() > 0x937 && archive.i32("has_texture")? != 0 {
        archive.scope("texture", texture)?;
    }
    material_rendering(archive)
}

fn material_rendering(archive: &mut impl MaterialRead) -> Result<()> {
    material_render_flags(archive)?;
    if archive.version() > 0xa05 {
        archive.text("classification")?;
    }
    if archive.version() > 0xdb3 {
        archive.i32("use_new_shaders")?;
    }
    if archive.version() > 0xf1d && archive.i32("has_rv_app_props")? != 0 {
        archive.scope("rv_app_props", rv)?;
    }
    material_extensions(archive)
}

fn material_extensions(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() > 0x3368 {
        archive.text("plm_id")?;
    }
    if archive.version() > 0x372a && archive.i32("has_texture2")? != 0 {
        archive.scope("texture2", texture)?;
    }
    archive.scope("user_property", super::material_properties::user_properties)
}

pub(crate) fn texture(archive: &mut impl MaterialRead) -> Result<()> {
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
    texture_tail(archive)
}

fn texture_tail(archive: &mut impl MaterialRead) -> Result<()> {
    archive.f64s(&["unknown_60", "texture_angle"])?;
    archive.text("unknown_70")?;
    archive.i16("texture_render_mode")?;
    archive.i32("ignore_color_material")?;
    if archive.version() > 0xf1d {
        archive.f64s(&["texture_trans_x", "texture_trans_y"])?;
    }
    if archive.version() > 0x372a {
        archive.i32("for_c_thread")?;
    }
    Ok(())
}

fn rv(archive: &mut impl MaterialRead) -> Result<()> {
    archive.u32("unknown_0")?;
    archive.u32("unknown_4")?;
    archive.text("unknown_8")?;
    archive.u8("texture_mapping_mode")?;
    archive.u32("material_color2")?;
    for prefix in ["u_dir", "v_dir", "center"] {
        float3(archive, prefix)?;
    }
    archive.f32("face_width")?;
    rv_color_parameters(archive)
}

fn rv_color_parameters(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() >= 0x1052 {
        archive.i32("color_form")?;
        archive.f32("reflectivity")?;
    }
    if archive.version() > 0x10a1 {
        archive.f32("color_mix")?;
    }
    rv_illumination(archive)
}

fn rv_illumination(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() > 0x10bf {
        archive.f32s(&["orig_ambient", "orig_diffuse", "orig_specular"])?;
        archive.i32("illum_shader_type")?;
    }
    if archive.version() >= 0x1165 {
        rv_shader_settings(archive)?;
    }
    rv_tail(archive)
}

fn rv_shader_settings(archive: &mut impl MaterialRead) -> Result<()> {
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
    Ok(())
}

fn rv_tail(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() <= 0x1179 {
        return rv_surface_details(archive);
    }
    archive.f32("refraction_roughness")?;
    archive.i32("surface_finish_shader_type")?;
    float3(archive, "appearance_u_dir")?;
    float3(archive, "appearance_v_dir")?;
    archive.f32s(&["appearance_trans_x", "appearance_trans_y"])?;
    rv_surface_details(archive)
}

fn rv_surface_details(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() > 0x119e {
        archive.f32s(&["knurl_height_cap", "knurl_angle"])?;
    }
    if archive.version() > 0x11ac {
        archive.f32("dimple_size")?;
    }
    rv_historical_dimensions(archive)?;
    if (0x3348..0x46ba).contains(&archive.version()) {
        archive.scope("unknown_symbol", legacy_symbol)?;
    }
    Ok(())
}

fn rv_historical_dimensions(archive: &mut impl MaterialRead) -> Result<()> {
    let version = archive.version();
    if version > 0x1415 {
        archive.f32s(&["default_width", "default_height"])?;
    }
    if version > 0x1421 {
        archive.text("lxp_filename")?;
    }
    if version > 0x1462 {
        archive.f32("round_sharp_edges")?;
    }
    Ok(())
}

fn legacy_symbol(archive: &mut impl MaterialRead) -> Result<()> {
    for index in 0..3 {
        archive.text(&format!("strings[{index}]"))?;
    }
    for index in 0..17 {
        archive.f32(&format!("floats[{index}]"))?;
    }
    for index in 0..2 {
        archive.u32(&format!("uints[{index}]"))?;
    }
    for index in 0..4 {
        archive.i32(&format!("ints[{index}]"))?;
    }
    Ok(())
}

pub(crate) fn float3(archive: &mut impl MaterialRead, prefix: &str) -> Result<()> {
    for index in 0..3 {
        archive.f32(&format!("{prefix}[{index}]"))?;
    }
    Ok(())
}

fn material_render_flags(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() <= 0x954 {
        return Ok(());
    }
    archive.i32s(&[
        "use_advanced_graphics",
        "use_defined_texture",
        "use_defined_shader",
        "use_defined_color",
        "use_defined_hatch",
    ])?;
    Ok(())
}
