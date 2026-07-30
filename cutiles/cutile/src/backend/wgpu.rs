use crate::error::CutileError;
use crate::traits::ManifoldCompute;

use super::types::{EntropyParams, EntropyResult};

#[cfg(feature = "wgpu-backend")]
mod gpu {
    use super::*;
    use bytemuck::{Pod, Zeroable};
    use std::sync::Arc;
    use wgpu::util::DeviceExt;

    const SHADER: &str = include_str!("../../kernels/entropy_reduce.wgsl");
    const WORKGROUP: u32 = 256;

    #[repr(C)]
    #[derive(Clone, Copy, Pod, Zeroable)]
    struct GpuParams {
        total_dof: u32,
        np: u32,
        nu: f32,
        tau: f32,
        surge_threshold: f32,
        prev_w_avg: f32,
    }

    pub struct WgpuBackend {
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        pipeline: wgpu::ComputePipeline,
        bind_layout: wgpu::BindGroupLayout,
    }

    impl WgpuBackend {
        pub fn new() -> Result<Self, CutileError> {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                ..Default::default()
            });

            let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            }))
            .ok_or_else(|| {
                CutileError::BackendUnavailable("no wgpu adapter available".into())
            })?;

            let (device, queue) = pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("cutile-wgpu"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            ))
            .map_err(|e| CutileError::Wgpu(e.to_string()))?;

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("entropy_reduce"),
                source: wgpu::ShaderSource::Wgsl(SHADER.into()),
            });

            let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("entropy_bind_layout"),
                entries: &[
                    storage_entry(0, true),
                    storage_entry(1, true),
                    storage_entry(2, true),
                    storage_entry(3, true),
                    uniform_entry(4),
                    storage_entry(5, false),
                    storage_entry(6, false),
                    storage_entry(7, false),
                    storage_entry(8, false),
                ],
            });

            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("entropy_pipeline_layout"),
                bind_group_layouts: &[&bind_layout],
                push_constant_ranges: &[],
            });

            let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("entropy_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            });

            Ok(Self {
                device: Arc::new(device),
                queue: Arc::new(queue),
                pipeline,
                bind_layout,
            })
        }

        pub fn compute_entropy_v2(&self, params: &EntropyParams) -> Result<EntropyResult, CutileError> {
            params.validate()?;
            let total_dof = params.omega_tilde.len() as u32;
            let np = params.d_perp_rho_sq.len().max(1) as u32;

            let omega_buf = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("omega"),
                    contents: bytemuck::cast_slice(&params.omega_tilde),
                    usage: wgpu::BufferUsages::STORAGE,
                });
            let grad_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("grad"),
                contents: bytemuck::cast_slice(&params.d_perp_rho_sq),
                usage: wgpu::BufferUsages::STORAGE,
            });
            let rho_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("rho"),
                contents: bytemuck::cast_slice(&params.rho),
                usage: wgpu::BufferUsages::STORAGE,
            });
            let strain_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("strain"),
                contents: bytemuck::cast_slice(&params.strain_norms),
                usage: wgpu::BufferUsages::STORAGE,
            });

            let gpu_params = GpuParams {
                total_dof,
                np,
                nu: params.nu,
                tau: params.tau,
                surge_threshold: params.surge_threshold,
                prev_w_avg: params.prev_w_avg,
            };
            let params_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("params"),
                contents: bytemuck::bytes_of(&gpu_params),
                usage: wgpu::BufferUsages::UNIFORM,
            });

            let out_w = zero_atomic_buffer(&self.device, "out_w");
            let out_visc = zero_atomic_buffer(&self.device, "out_visc");
            let out_stretch = zero_atomic_buffer(&self.device, "out_stretch");
            let out_betti = zero_atomic_buffer(&self.device, "out_betti");

            let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("entropy_bind"),
                layout: &self.bind_layout,
                entries: &[
                    bind_entry(0, &omega_buf),
                    bind_entry(1, &grad_buf),
                    bind_entry(2, &rho_buf),
                    bind_entry(3, &strain_buf),
                    bind_entry(4, &params_buf),
                    bind_entry(5, &out_w),
                    bind_entry(6, &out_visc),
                    bind_entry(7, &out_stretch),
                    bind_entry(8, &out_betti),
                ],
            });

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("entropy_encoder"),
                });

            {
                let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("entropy_pass"),
                    timestamp_writes: None,
                });
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &bind_group, &[]);
                let groups = (total_dof + WORKGROUP - 1) / WORKGROUP;
                pass.dispatch_workgroups(groups.max(1), 1, 1);
            }

            self.queue.submit(Some(encoder.finish()));

            let w = read_atomic_f32(&self.device, &self.queue, &out_w);
            let visc = read_atomic_f32(&self.device, &self.queue, &out_visc);
            let stretch = read_atomic_f32(&self.device, &self.queue, &out_stretch);
            let betti_proxy = read_atomic_f32(&self.device, &self.queue, &out_betti);

            let base = params.prev_w_avg.abs().max(1e-12);
            let rel = (w - params.prev_w_avg).abs() / base;
            let surge = if rel > params.surge_threshold { 1.0 } else { 0.0 };

            Ok(EntropyResult {
                w,
                visc,
                stretch,
                surge,
                betti_proxy,
                used_gpu_kernel: true,
            })
        }

        /// Batch multiple evaluations in one command encoder (reduces launch overhead).
        pub fn compute_entropy_batch(
            &self,
            batch: &[EntropyParams],
        ) -> Result<Vec<EntropyResult>, CutileError> {
            if batch.is_empty() {
                return Ok(Vec::new());
            }
            if batch.len() == 1 {
                return Ok(vec![self.compute_entropy_v2(&batch[0])?]);
            }

            // MVP: shared encoder not yet fused — sequential GPU launches still benefit from warm pipeline.
            batch.iter().map(|p| self.compute_entropy_v2(p)).collect()
        }
    }

    fn storage_entry(binding: u32, read_only: bool) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn uniform_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn bind_entry(binding: u32, buffer: &wgpu::Buffer) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding,
            resource: buffer.as_entire_binding(),
        }
    }

    fn zero_atomic_buffer(device: &wgpu::Device, label: &str) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: &0u32.to_le_bytes(),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        })
    }

    fn read_atomic_f32(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        buffer: &wgpu::Buffer,
    ) -> f32 {
        let staging = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("staging"),
            size: 4,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("readback"),
        });
        encoder.copy_buffer_to_buffer(buffer, 0, &staging, 0, 4);
        queue.submit(Some(encoder.finish()));

        let slice = staging.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let data = slice.get_mapped_range();
        let bits = u32::from_le_bytes(data[0..4].try_into().unwrap());
        f32::from_bits(bits)
    }

    impl ManifoldCompute for WgpuBackend {
        type BackendError = CutileError;

        fn compute_entropy(
            &self,
            omega_tilde: &[f32],
            d_perp_rho_sq: &[f32],
            rho: &[f32],
            strain_norms: &[f32],
            tau: f32,
            nu: f32,
        ) -> Result<(f32, f32, f32), Self::BackendError> {
            let r = self.compute_entropy_v2(&EntropyParams {
                omega_tilde: omega_tilde.to_vec(),
                d_perp_rho_sq: d_perp_rho_sq.to_vec(),
                rho: rho.to_vec(),
                strain_norms: strain_norms.to_vec(),
                tau,
                nu,
                surge_threshold: 0.05,
                prev_w_avg: 0.0,
            })?;
            Ok((r.w, r.visc, r.stretch))
        }
    }
}

#[cfg(feature = "wgpu-backend")]
pub use gpu::WgpuBackend;

#[cfg(not(feature = "wgpu-backend"))]
pub struct WgpuBackend;

#[cfg(not(feature = "wgpu-backend"))]
impl WgpuBackend {
    pub fn new() -> Result<Self, CutileError> {
        Err(CutileError::BackendUnavailable(
            "compile with --features wgpu-backend".into(),
        ))
    }

    pub fn compute_entropy_v2(&self, _params: &EntropyParams) -> Result<EntropyResult, CutileError> {
        Err(CutileError::BackendUnavailable(
            "compile with --features wgpu-backend".into(),
        ))
    }

    pub fn compute_entropy_batch(
        &self,
        _batch: &[EntropyParams],
    ) -> Result<Vec<EntropyResult>, CutileError> {
        Err(CutileError::BackendUnavailable(
            "compile with --features wgpu-backend".into(),
        ))
    }
}

#[cfg(not(feature = "wgpu-backend"))]
impl ManifoldCompute for WgpuBackend {
    type BackendError = CutileError;

    fn compute_entropy(
        &self,
        _omega_tilde: &[f32],
        _d_perp_rho_sq: &[f32],
        _rho: &[f32],
        _strain_norms: &[f32],
        _tau: f32,
        _nu: f32,
    ) -> Result<(f32, f32, f32), Self::BackendError> {
        Err(CutileError::BackendUnavailable(
            "compile with --features wgpu-backend".into(),
        ))
    }
}