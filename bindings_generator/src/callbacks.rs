use bindgen::callbacks::{EnumVariantCustomBehavior, EnumVariantValue, ItemInfo, ParseCallbacks};

#[derive(Debug)]
pub struct GlslangCallbacks;

impl GlslangCallbacks {
    pub fn rename_resource_type(variant: &str) -> Option<String> {
        match variant {
            "GLSLANG_RESOURCE_TYPE_SAMPLER" => Some("Sampler".into()),
            "GLSLANG_RESOURCE_TYPE_TEXTURE" => Some("Texture".into()),
            "GLSLANG_RESOURCE_TYPE_IMAGE" => Some("Image".into()),
            "GLSLANG_RESOURCE_TYPE_UBO" => Some("UBO".into()),
            "GLSLANG_RESOURCE_TYPE_SSBO" => Some("SSBO".into()),
            "GLSLANG_RESOURCE_TYPE_UAV" => Some("UAV".into()),
            "GLSLANG_RESOURCE_TYPE_COUNT" => Some("Count".into()),
            _ => None,
        }
    }

    pub fn rename_shader_stage(variant: &str) -> Option<String> {
        match variant {
            "GLSLANG_STAGE_VERTEX" => Some("Vertex".into()),
            "GLSLANG_STAGE_TESSCONTROL" => Some("TesselationControl".into()),
            "GLSLANG_STAGE_TESSEVALUATION" => Some("TesselationEvaluation".into()),
            "GLSLANG_STAGE_GEOMETRY" => Some("Geometry".into()),
            "GLSLANG_STAGE_FRAGMENT" => Some("Fragment".into()),
            "GLSLANG_STAGE_COMPUTE" => Some("Compute".into()),
            "GLSLANG_STAGE_KERNEL" => Some("Kernel".into()),
            "GLSLANG_STAGE_RAYGEN" => Some("RayGeneration".into()),
            "GLSLANG_STAGE_INTERSECT" => Some("Intersect".into()),
            "GLSLANG_STAGE_ANYHIT" => Some("AnyHit".into()),
            "GLSLANG_STAGE_CLOSESTHIT" => Some("ClosestHit".into()),
            "GLSLANG_STAGE_MISS" => Some("Miss".into()),
            "GLSLANG_STAGE_CALLABLE" => Some("Callable".into()),
            "GLSLANG_STAGE_TASK" => Some("Task".into()),
            "GLSLANG_STAGE_MESH" => Some("Mesh".into()),
            "GLSLANG_STAGE_COUNT" => Some("Count".into()),
            _ => None,
        }
    }

    pub fn rename_source_type(variant: &str) -> Option<String> {
        match variant {
            "GLSLANG_SOURCE_NONE" => Some("None".into()),
            "GLSLANG_SOURCE_GLSL" => Some("GLSL".into()),
            "GLSLANG_SOURCE_HLSL" => Some("HLSL".into()),
            _ => None,
        }
    }

    pub fn rename_client_type(variant: &str) -> Option<String> {
        match variant {
            "GLSLANG_CLIENT_NONE" => Some("None".into()),
            "GLSLANG_CLIENT_VULKAN" => Some("Vulkan".into()),
            "GLSLANG_CLIENT_OPENGL" => Some("OpenGL".into()),
            _ => None,
        }
    }

    pub fn rename_shader_stage_mask(variant: &str) -> Option<String> {
        Some(variant.replace("GLSLANG_STAGE_", "").replace("_MASK", ""))
    }

    pub fn rename_target_language(variant: &str) -> Option<String> {
        match variant {
            "GLSLANG_TARGET_NONE" => Some("None".into()),
            "GLSLANG_TARGET_SPV" => Some("SPIRV".into()),
            _ => None,
        }
    }

    pub fn rename_target_language_version(variant: &str) -> Option<String> {
        Some(variant.replace("GLSLANG_TARGET_SPV_", "SPIRV"))
    }
    pub fn rename_target_client_version(variant: &str) -> Option<String> {
        if variant.starts_with("GLSLANG_TARGET_VULKAN_") {
            Some(variant.replace("GLSLANG_TARGET_VULKAN_", "Vulkan"))
        } else if variant.starts_with("GLSLANG_TARGET_OPENGL") {
            Some(variant.replace("GLSLANG_TARGET_OPENGL_", "OpenGL"))
        } else {
            None
        }
    }

    pub fn rename_messages(variant: &str) -> Option<String> {
        Some(variant.replace("GLSLANG_MSG_", "").replace("_BIT", ""))
    }

    pub fn rename_shader_options(variant: &str) -> Option<String> {
        Some(variant.replace("GLSLANG_SHADER_", "").replace("_BIT", ""))
    }
    pub fn rename_texture_sampler_transform_mode(variant: &str) -> Option<String> {
        match variant {
            "GLSLANG_TEX_SAMP_TRANS_KEEP" => Some("Keep".into()),
            "GLSLANG_TEX_SAMP_TRANS_UPGRADE_TEXTURE_REMOVE_SAMPLER" => {
                Some("UpgradeTextureRemoveSampler".into())
            }
            _ => None,
        }
    }

    pub fn rename_profile(variant: &str) -> Option<String> {
        match variant {
            "GLSLANG_BAD_PROFILE" => Some("Bad".into()),
            "GLSLANG_NO_PROFILE" => Some("None".into()),
            "GLSLANG_CORE_PROFILE" => Some("Core".into()),
            "GLSLANG_COMPATIBILITY_PROFILE" => Some("Compatibility".into()),
            "GLSLANG_ES_PROFILE" => Some("ES".into()),
            _ => None,
        }
    }
}
impl ParseCallbacks for GlslangCallbacks {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        match enum_name {
            Some("glslang_stage_t") => Self::rename_shader_stage(original_variant_name),
            Some("glslang_stage_mask_t") => Self::rename_shader_stage_mask(original_variant_name),
            Some("glslang_resource_type_t") => Self::rename_resource_type(original_variant_name),
            Some("glslang_source_t") => Self::rename_source_type(original_variant_name),
            Some("glslang_client_t") => Self::rename_client_type(original_variant_name),
            Some("glslang_target_language_t") => {
                Self::rename_target_language(original_variant_name)
            }
            Some("glslang_target_client_version_t") => {
                Self::rename_target_client_version(original_variant_name)
            }
            Some("glslang_target_language_version_t") => {
                Self::rename_target_language_version(original_variant_name)
            }
            Some("glslang_texture_sampler_transform_mode_t") => {
                Self::rename_texture_sampler_transform_mode(original_variant_name)
            }
            Some("glslang_messages_t") => Self::rename_messages(original_variant_name),
            Some("glslang_reflection_options_t") => Self::rename_messages(original_variant_name),
            Some("glslang_shader_options_t") => Self::rename_shader_options(original_variant_name),
            Some("glslang_profile_t") => Self::rename_profile(original_variant_name),
            // Some("enum dxil_validator_version") => {
            //     Self::rename_validator_version(original_variant_name, variant_value)
            // }
            // Some("enum dxil_spirv_yz_flip_mode") => {
            //     Self::rename_flip_mode(original_variant_name)
            // }
            _ => {
                eprintln!("skipping {:?}", enum_name);
                None
            }
        }
    }

    fn enum_variant_behavior(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<EnumVariantCustomBehavior> {
        if original_variant_name.ends_with("_COUNT") {
            return Some(EnumVariantCustomBehavior::Hide);
        };

        match enum_name {
            Some("glslang_stage_t") => {
                if original_variant_name.ends_with("_NV") {
                    return Some(EnumVariantCustomBehavior::Hide);
                }
                None
            }
            Some("glslang_stage_mask_t") => {
                if original_variant_name.ends_with("_NV_MASK") {
                    return Some(EnumVariantCustomBehavior::Hide);
                }
                None
            }
            Some("glslang_profile_t") => {
                if original_variant_name == "GLSLANG_BAD_PROFILE" {
                    return Some(EnumVariantCustomBehavior::Hide);
                }
                None
            }
            _ => None,
        }
    }
    fn item_name(&self, item_info: ItemInfo) -> Option<String> {
        match item_info.name {
            "GLSLANG_STAGE_TASK_NV" => {
                panic!("huh")
            }
            _ => None,
        }
    }
}
