use glslang_sys as sys;
use glslang_sys::glslang_resource_s__bindgen_ty_1;

#[repr(transparent)]
pub struct Limits(sys::glslang_limits_t);

#[repr(transparent)]
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
            __bindgen_anon_1: glslang_resource_s__bindgen_ty_1 {
                max_dual_source_draw_buffers_ext: 1,
            },
            limits: Limits::default().0,
        })
    }
}

impl Default for Limits {
    fn default() -> Self {
        Self(sys::glslang_limits_t {
            non_inductive_for_loops: true,
            while_loops: true,
            do_while_loops: true,
            general_uniform_indexing: true,
            general_attribute_matrix_vector_indexing: true,
            general_varying_indexing: true,
            general_sampler_indexing: true,
            general_variable_indexing: true,
            general_constant_matrix_vector_indexing: true,
        })
    }
}
impl Limits {
    pub fn new() -> Self {
        Self::default()
    }

    // todo: setters
}
