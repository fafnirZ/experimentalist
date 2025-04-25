// influenced by: https://github.com/junglie85/wgpu-samples/blob/main/samples/hello-triangle/main.rs
// influenced by: https://github.com/gfx-rs/wgpu/blob/trunk/examples/standalone/02_hello_window/src/main.rs

use std::{num::NonZero, sync::Arc};

use wgpu::{
    BindGroupLayoutDescriptor, Buffer, BufferDescriptor, BufferUsages, CommandEncoder, Device,
    FragmentState, MultisampleState, PipelineCache, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, RenderPass, RenderPipeline, RenderPipelineDescriptor,
    SurfaceTexture, Texture, TextureFormat, TextureView, VertexAttribute, VertexBufferLayout,
    VertexFormat, VertexState, VertexStepMode, include_wgsl,
};
use winit::window::Window;

pub struct State {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,

    // NEW!
    render_pipeline: RenderPipeline,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }

    fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        }
    }
}

impl State {
    pub async fn new(window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let render_pipeline = init_shaders(&device, &surface_format);

        let state = State {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,

            render_pipeline,
        };

        // Configure surface for the first time
        state.configure_surface();

        state
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        // reconfigure the surface
        self.configure_surface();
    }

    pub fn render(&mut self) {
        // Create texture view
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        // Renders a GREEN screen
        let mut encoder = self.device.create_command_encoder(&Default::default());

        self.render_shader(&texture_view, &mut encoder);

        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }

    fn init_vertices(&mut self) -> [Vertex; 3] {
        // create vertices
        let vertices = [
            Vertex::new(0.0, 0.5),
            Vertex::new(-0.5, -0.5),
            Vertex::new(0.5, -0.5),
        ];
        return vertices;
    }

    fn init_vertex_buffers(&mut self, device: &Device) -> Buffer {
        // create buffers
        let vbo = device.create_buffer(&BufferDescriptor {
            label: None,
            size: size_of::<Vertex>() as u64 * 3,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return vbo;
    }

    fn render_shader(&mut self, texture_view: &TextureView, encoder: &mut CommandEncoder) {
        // Create the renderpass which will clear the screen.
        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // If you wanted to call any drawing commands, they would go here.
        renderpass.set_pipeline(&self.render_pipeline);
        renderpass.draw(0..3, 0..1);

        drop(renderpass);
    }
}

// needs to be created during initialisation of state (init fn)
fn init_shaders(device: &wgpu::Device, surface_format: &TextureFormat) -> RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let shader = device.create_shader_module(include_wgsl!("shaders/line.wgsl"));
    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: PipelineCompilationOptions::default(),
        },
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                // 4.
                format: *surface_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        multisample: MultisampleState::default(),
        cache: None,
        multiview: None,
    });
    return render_pipeline;
}
