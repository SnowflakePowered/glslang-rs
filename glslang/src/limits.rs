use glslang_sys as sys;

pub use sys::glslang_limits_t as CompilerLimits;
use crate::Compiler;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ResourceLimits(pub(crate) sys::glslang_resource_t);

impl Default for ResourceLimits {
    fn default() -> Self {
        Self(sys::glslang_resource_t {
            max_lights: 32,
            max_clip_planes: 6,
            max_texture_units: 32,
            max_texture_coords: 32,
            max_vertex_attribs: 64,
            max_vertex_uniform_components: 4096,
            max_varying_floats: 64,
            max_vertex_texture_image_units: 32,
            max_combined_texture_image_units: 80,
            max_texture_image_units: 32,
            max_fragment_uniform_components: 4096,
            max_draw_buffers: 32,
            max_vertex_uniform_vectors: 128,
            max_varying_vectors: 8,
            max_fragment_uniform_vectors: 16,
            max_vertex_output_vectors: 16,
            max_fragment_input_vectors: 15,
            min_program_texel_offset: -8,
            max_program_texel_offset: 7,
            max_clip_distances: 8,
            max_compute_work_group_count_x: 65535,
            max_compute_work_group_count_y: 65535,
            max_compute_work_group_count_z: 65535,
            max_compute_work_group_size_x: 1024,
            max_compute_work_group_size_y: 1024,
            max_compute_work_group_size_z: 64,
            max_compute_uniform_components: 1024,
            max_compute_texture_image_units: 16,
            max_compute_image_uniforms: 8,
            max_compute_atomic_counters: 8,
            max_compute_atomic_counter_buffers: 1,
            max_varying_components: 60,
            max_vertex_output_components: 64,
            max_geometry_input_components: 64,
            max_geometry_output_components: 128,
            max_fragment_input_components: 128,
            max_image_units: 8,
            max_combined_image_units_and_fragment_outputs: 8,
            max_combined_shader_output_resources: 8,
            max_image_samples: 0,
            max_vertex_image_uniforms: 0,
            max_tess_control_image_uniforms: 0,
            max_tess_evaluation_image_uniforms: 0,
            max_geometry_image_uniforms: 0,
            max_fragment_image_uniforms: 8,
            max_combined_image_uniforms: 8,
            max_geometry_texture_image_units: 16,
            max_geometry_output_vertices: 256,
            max_geometry_total_output_components: 1024,
            max_geometry_uniform_components: 64,
            max_geometry_varying_components: 128,
            max_tess_control_input_components: 128,
            max_tess_control_output_components: 16,
            max_tess_control_texture_image_units: 1,
            max_tess_control_uniform_components: 1024,
            max_tess_control_total_output_components: 4096,
            max_tess_evaluation_input_components: 128,
            max_tess_evaluation_output_components: 128,
            max_tess_evaluation_texture_image_units: 16,
            max_tess_evaluation_uniform_components: 1024,
            max_tess_patch_components: 0,
            max_patch_vertices: 32,
            max_tess_gen_level: 64,
            max_viewports: 16,
            max_vertex_atomic_counters: 0,
            max_tess_control_atomic_counters: 0,
            max_tess_evaluation_atomic_counters: 0,
            max_geometry_atomic_counters: 0,
            max_fragment_atomic_counters: 8,
            max_combined_atomic_counters: 8,
            max_atomic_counter_bindings: 1,
            max_vertex_atomic_counter_buffers: 0,
            max_tess_control_atomic_counter_buffers: 0,
            max_tess_evaluation_atomic_counter_buffers: 0,
            max_geometry_atomic_counter_buffers: 0,
            max_fragment_atomic_counter_buffers: 1,
            max_combined_atomic_counter_buffers: 1,
            max_atomic_counter_buffer_size: 16384,
            max_transform_feedback_buffers: 4,
            max_transform_feedback_interleaved_components: 64,
            max_cull_distances: 8,
            max_combined_clip_and_cull_distances: 8,
            max_samples: 4,
            max_mesh_output_vertices_nv: 256,
            max_mesh_output_primitives_nv: 512,
            max_mesh_work_group_size_x_nv: 32,
            max_mesh_work_group_size_y_nv: 1,
            max_mesh_work_group_size_z_nv: 1,
            max_task_work_group_size_x_nv: 32,
            max_task_work_group_size_y_nv: 1,
            max_task_work_group_size_z_nv: 1,
            max_mesh_view_count_nv: 4,
            max_mesh_output_vertices_ext: 256,
            max_mesh_output_primitives_ext: 256,
            max_mesh_work_group_size_x_ext: 128,
            max_mesh_work_group_size_y_ext: 128,
            max_mesh_work_group_size_z_ext: 128,
            max_task_work_group_size_x_ext: 128,
            max_task_work_group_size_y_ext: 128,
            max_task_work_group_size_z_ext: 128,
            max_mesh_view_count_ext: 4,
            __bindgen_anon_1: sys::glslang_resource_s__bindgen_ty_1 {
                max_dual_source_draw_buffers_ext: 1,
            },
            limits: CompilerLimits::default(),
        })
    }
}

impl ResourceLimits {
    pub fn compiler_limits_mut(&mut self) -> &mut CompilerLimits {
        &mut self.0.limits
    }

    pub fn set_max_dual_source_draw_buffers_ext(&mut self, value: i32) {
        self.0.__bindgen_anon_1.max_dual_source_draw_buffers_ext = value;
    }

    pub fn set_max_lights(&mut self, value: i32) {
        self.0.max_lights = value;
    }
    pub fn set_max_clip_planes(&mut self, value: i32) {
        self.0.max_clip_planes = value;
    }
    pub fn set_max_texture_units(&mut self, value: i32) {
        self.0.max_texture_units = value;
    }
    pub fn set_max_texture_coords(&mut self, value: i32) {
        self.0.max_texture_coords = value;
    }
    pub fn set_max_vertex_attribs(&mut self, value: i32) {
        self.0.max_vertex_attribs = value;
    }
    pub fn set_max_vertex_uniform_components(&mut self, value: i32) {
        self.0.max_vertex_uniform_components = value;
    }
    pub fn set_max_varying_floats(&mut self, value: i32) {
        self.0.max_varying_floats = value;
    }
    pub fn set_max_vertex_texture_image_units(&mut self, value: i32) {
        self.0.max_vertex_texture_image_units = value;
    }
    pub fn set_max_combined_texture_image_units(&mut self, value: i32) {
        self.0.max_combined_texture_image_units = value;
    }
    pub fn set_max_texture_image_units(&mut self, value: i32) {
        self.0.max_texture_image_units = value;
    }
    pub fn set_max_fragment_uniform_components(&mut self, value: i32) {
        self.0.max_fragment_uniform_components = value;
    }
    pub fn set_max_draw_buffers(&mut self, value: i32) {
        self.0.max_draw_buffers = value;
    }
    pub fn set_max_vertex_uniform_vectors(&mut self, value: i32) {
        self.0.max_vertex_uniform_vectors = value;
    }
    pub fn set_max_varying_vectors(&mut self, value: i32) {
        self.0.max_varying_vectors = value;
    }
    pub fn set_max_fragment_uniform_vectors(&mut self, value: i32) {
        self.0.max_fragment_uniform_vectors = value;
    }
    pub fn set_max_vertex_output_vectors(&mut self, value: i32) {
        self.0.max_vertex_output_vectors = value;
    }
    pub fn set_max_fragment_input_vectors(&mut self, value: i32) {
        self.0.max_fragment_input_vectors = value;
    }
    pub fn set_min_program_texel_offset(&mut self, value: i32) {
        self.0.min_program_texel_offset = value;
    }
    pub fn set_max_program_texel_offset(&mut self, value: i32) {
        self.0.max_program_texel_offset = value;
    }
    pub fn set_max_clip_distances(&mut self, value: i32) {
        self.0.max_clip_distances = value;
    }
    pub fn set_max_compute_work_group_count_x(&mut self, value: i32) {
        self.0.max_compute_work_group_count_x = value;
    }
    pub fn set_max_compute_work_group_count_y(&mut self, value: i32) {
        self.0.max_compute_work_group_count_y = value;
    }
    pub fn set_max_compute_work_group_count_z(&mut self, value: i32) {
        self.0.max_compute_work_group_count_z = value;
    }
    pub fn set_max_compute_work_group_size_x(&mut self, value: i32) {
        self.0.max_compute_work_group_size_x = value;
    }
    pub fn set_max_compute_work_group_size_y(&mut self, value: i32) {
        self.0.max_compute_work_group_size_y = value;
    }
    pub fn set_max_compute_work_group_size_z(&mut self, value: i32) {
        self.0.max_compute_work_group_size_z = value;
    }
    pub fn set_max_compute_uniform_components(&mut self, value: i32) {
        self.0.max_compute_uniform_components = value;
    }
    pub fn set_max_compute_texture_image_units(&mut self, value: i32) {
        self.0.max_compute_texture_image_units = value;
    }
    pub fn set_max_compute_image_uniforms(&mut self, value: i32) {
        self.0.max_compute_image_uniforms = value;
    }
    pub fn set_max_compute_atomic_counters(&mut self, value: i32) {
        self.0.max_compute_atomic_counters = value;
    }
    pub fn set_max_compute_atomic_counter_buffers(&mut self, value: i32) {
        self.0.max_compute_atomic_counter_buffers = value;
    }
    pub fn set_max_varying_components(&mut self, value: i32) {
        self.0.max_varying_components = value;
    }
    pub fn set_max_vertex_output_components(&mut self, value: i32) {
        self.0.max_vertex_output_components = value;
    }
    pub fn set_max_geometry_input_components(&mut self, value: i32) {
        self.0.max_geometry_input_components = value;
    }
    pub fn set_max_geometry_output_components(&mut self, value: i32) {
        self.0.max_geometry_output_components = value;
    }
    pub fn set_max_fragment_input_components(&mut self, value: i32) {
        self.0.max_fragment_input_components = value;
    }
    pub fn set_max_image_units(&mut self, value: i32) {
        self.0.max_image_units = value;
    }
    pub fn set_max_combined_image_units_and_fragment_outputs(&mut self, value: i32) {
        self.0.max_combined_image_units_and_fragment_outputs = value;
    }
    pub fn set_max_combined_shader_output_resources(&mut self, value: i32) {
        self.0.max_combined_shader_output_resources = value;
    }
    pub fn set_max_image_samples(&mut self, value: i32) {
        self.0.max_image_samples = value;
    }
    pub fn set_max_vertex_image_uniforms(&mut self, value: i32) {
        self.0.max_vertex_image_uniforms = value;
    }
    pub fn set_max_tess_control_image_uniforms(&mut self, value: i32) {
        self.0.max_tess_control_image_uniforms = value;
    }
    pub fn set_max_tess_evaluation_image_uniforms(&mut self, value: i32) {
        self.0.max_tess_evaluation_image_uniforms = value;
    }
    pub fn set_max_geometry_image_uniforms(&mut self, value: i32) {
        self.0.max_geometry_image_uniforms = value;
    }
    pub fn set_max_fragment_image_uniforms(&mut self, value: i32) {
        self.0.max_fragment_image_uniforms = value;
    }
    pub fn set_max_combined_image_uniforms(&mut self, value: i32) {
        self.0.max_combined_image_uniforms = value;
    }
    pub fn set_max_geometry_texture_image_units(&mut self, value: i32) {
        self.0.max_geometry_texture_image_units = value;
    }
    pub fn set_max_geometry_output_vertices(&mut self, value: i32) {
        self.0.max_geometry_output_vertices = value;
    }
    pub fn set_max_geometry_total_output_components(&mut self, value: i32) {
        self.0.max_geometry_total_output_components = value;
    }
    pub fn set_max_geometry_uniform_components(&mut self, value: i32) {
        self.0.max_geometry_uniform_components = value;
    }
    pub fn set_max_geometry_varying_components(&mut self, value: i32) {
        self.0.max_geometry_varying_components = value;
    }
    pub fn set_max_tess_control_input_components(&mut self, value: i32) {
        self.0.max_tess_control_input_components = value;
    }
    pub fn set_max_tess_control_output_components(&mut self, value: i32) {
        self.0.max_tess_control_output_components = value;
    }
    pub fn set_max_tess_control_texture_image_units(&mut self, value: i32) {
        self.0.max_tess_control_texture_image_units = value;
    }
    pub fn set_max_tess_control_uniform_components(&mut self, value: i32) {
        self.0.max_tess_control_uniform_components = value;
    }
    pub fn set_max_tess_control_total_output_components(&mut self, value: i32) {
        self.0.max_tess_control_total_output_components = value;
    }
    pub fn set_max_tess_evaluation_input_components(&mut self, value: i32) {
        self.0.max_tess_evaluation_input_components = value;
    }
    pub fn set_max_tess_evaluation_output_components(&mut self, value: i32) {
        self.0.max_tess_evaluation_output_components = value;
    }
    pub fn set_max_tess_evaluation_texture_image_units(&mut self, value: i32) {
        self.0.max_tess_evaluation_texture_image_units = value;
    }
    pub fn set_max_tess_evaluation_uniform_components(&mut self, value: i32) {
        self.0.max_tess_evaluation_uniform_components = value;
    }
    pub fn set_max_tess_patch_components(&mut self, value: i32) {
        self.0.max_tess_patch_components = value;
    }
    pub fn set_max_patch_vertices(&mut self, value: i32) {
        self.0.max_patch_vertices = value;
    }
    pub fn set_max_tess_gen_level(&mut self, value: i32) {
        self.0.max_tess_gen_level = value;
    }
    pub fn set_max_viewports(&mut self, value: i32) {
        self.0.max_viewports = value;
    }
    pub fn set_max_vertex_atomic_counters(&mut self, value: i32) {
        self.0.max_vertex_atomic_counters = value;
    }
    pub fn set_max_tess_control_atomic_counters(&mut self, value: i32) {
        self.0.max_tess_control_atomic_counters = value;
    }
    pub fn set_max_tess_evaluation_atomic_counters(&mut self, value: i32) {
        self.0.max_tess_evaluation_atomic_counters = value;
    }
    pub fn set_max_geometry_atomic_counters(&mut self, value: i32) {
        self.0.max_geometry_atomic_counters = value;
    }
    pub fn set_max_fragment_atomic_counters(&mut self, value: i32) {
        self.0.max_fragment_atomic_counters = value;
    }
    pub fn set_max_combined_atomic_counters(&mut self, value: i32) {
        self.0.max_combined_atomic_counters = value;
    }
    pub fn set_max_atomic_counter_bindings(&mut self, value: i32) {
        self.0.max_atomic_counter_bindings = value;
    }
    pub fn set_max_vertex_atomic_counter_buffers(&mut self, value: i32) {
        self.0.max_vertex_atomic_counter_buffers = value;
    }
    pub fn set_max_tess_control_atomic_counter_buffers(&mut self, value: i32) {
        self.0.max_tess_control_atomic_counter_buffers = value;
    }
    pub fn set_max_tess_evaluation_atomic_counter_buffers(&mut self, value: i32) {
        self.0.max_tess_evaluation_atomic_counter_buffers = value;
    }
    pub fn set_max_geometry_atomic_counter_buffers(&mut self, value: i32) {
        self.0.max_geometry_atomic_counter_buffers = value;
    }
    pub fn set_max_fragment_atomic_counter_buffers(&mut self, value: i32) {
        self.0.max_fragment_atomic_counter_buffers = value;
    }
    pub fn set_max_combined_atomic_counter_buffers(&mut self, value: i32) {
        self.0.max_combined_atomic_counter_buffers = value;
    }
    pub fn set_max_atomic_counter_buffer_size(&mut self, value: i32) {
        self.0.max_atomic_counter_buffer_size = value;
    }
    pub fn set_max_transform_feedback_buffers(&mut self, value: i32) {
        self.0.max_transform_feedback_buffers = value;
    }
    pub fn set_max_transform_feedback_interleaved_components(&mut self, value: i32) {
        self.0.max_transform_feedback_interleaved_components = value;
    }
    pub fn set_max_cull_distances(&mut self, value: i32) {
        self.0.max_cull_distances = value;
    }
    pub fn set_max_combined_clip_and_cull_distances(&mut self, value: i32) {
        self.0.max_combined_clip_and_cull_distances = value;
    }
    pub fn set_max_samples(&mut self, value: i32) {
        self.0.max_samples = value;
    }
    pub fn set_max_mesh_output_vertices_nv(&mut self, value: i32) {
        self.0.max_mesh_output_vertices_nv = value;
    }
    pub fn set_max_mesh_output_primitives_nv(&mut self, value: i32) {
        self.0.max_mesh_output_primitives_nv = value;
    }
    pub fn set_max_mesh_work_group_size_x_nv(&mut self, value: i32) {
        self.0.max_mesh_work_group_size_x_nv = value;
    }
    pub fn set_max_mesh_work_group_size_y_nv(&mut self, value: i32) {
        self.0.max_mesh_work_group_size_y_nv = value;
    }
    pub fn set_max_mesh_work_group_size_z_nv(&mut self, value: i32) {
        self.0.max_mesh_work_group_size_z_nv = value;
    }
    pub fn set_max_task_work_group_size_x_nv(&mut self, value: i32) {
        self.0.max_task_work_group_size_x_nv = value;
    }
    pub fn set_max_task_work_group_size_y_nv(&mut self, value: i32) {
        self.0.max_task_work_group_size_y_nv = value;
    }
    pub fn set_max_task_work_group_size_z_nv(&mut self, value: i32) {
        self.0.max_task_work_group_size_z_nv = value;
    }
    pub fn set_max_mesh_view_count_nv(&mut self, value: i32) {
        self.0.max_mesh_view_count_nv = value;
    }
    pub fn set_max_mesh_output_vertices_ext(&mut self, value: i32) {
        self.0.max_mesh_output_vertices_ext = value;
    }
    pub fn set_max_mesh_output_primitives_ext(&mut self, value: i32) {
        self.0.max_mesh_output_primitives_ext = value;
    }
    pub fn set_max_mesh_work_group_size_x_ext(&mut self, value: i32) {
        self.0.max_mesh_work_group_size_x_ext = value;
    }
    pub fn set_max_mesh_work_group_size_y_ext(&mut self, value: i32) {
        self.0.max_mesh_work_group_size_y_ext = value;
    }
    pub fn set_max_mesh_work_group_size_z_ext(&mut self, value: i32) {
        self.0.max_mesh_work_group_size_z_ext = value;
    }
    pub fn set_max_task_work_group_size_x_ext(&mut self, value: i32) {
        self.0.max_task_work_group_size_x_ext = value;
    }
    pub fn set_max_task_work_group_size_y_ext(&mut self, value: i32) {
        self.0.max_task_work_group_size_y_ext = value;
    }
    pub fn set_max_task_work_group_size_z_ext(&mut self, value: i32) {
        self.0.max_task_work_group_size_z_ext = value;
    }
    pub fn set_max_mesh_view_count_ext(&mut self, value: i32) {
        self.0.max_mesh_view_count_ext = value;
    }
}
