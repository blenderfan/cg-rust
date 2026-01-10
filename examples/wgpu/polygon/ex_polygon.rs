
use std::sync::Arc;
use std::f32::consts;

use num_traits::clamp;

use bytemuck::{Pod, Zeroable};
use cg_rust::polygon::{Polygon, PolygonFloat};
use cg_rust::cardinal_direction::CardinalDirection;
use cg_rust::util::*;
use cg_rust::vector::Vec2;
use cg_rust::vector::Vec2f;
use cg_rust::vector::Vec3;
use cg_rust::vector::Vec3f;
use wgpu::{util::DeviceExt};
use winit::event::MouseScrollDelta;
use winit::{
    event::WindowEvent, application::ApplicationHandler,
    event_loop::{ActiveEventLoop,ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct Vertex {
    pos: [f32; 3],
}

struct Entity {
    vertex_count: u32,
    vertex_buf: wgpu::Buffer,
}

struct Camera {
    screen_size: (u32, u32),
    angle_y: f32,
    angle_xz: f32,
    dist: f32
}

impl Camera {
    fn to_uniform_data(&self) -> [f32; 16 * 3 + 4] {
        let aspect = self.screen_size.0 as f32 / self.screen_size.1 as f32;
        let proj = glam::Mat4::perspective_rh(consts::FRAC_PI_4, aspect, 1.0, 50.0);
        let cam_pos = glam::Vec3::new(
            self.angle_xz.cos() * self.angle_y.sin() * self.dist,
            self.angle_xz.sin() * self.dist,
            self.angle_xz.cos() * self.angle_y.cos() * self.dist,
        );
        let view = glam::Mat4::look_at_rh(
            cam_pos,
            glam::Vec3::new(0f32, 0.0, 0.0),
            glam::Vec3::Y,
        );
        let proj_inv = proj.inverse();

        let mut raw = [0f32; 16 * 3 + 4];
        raw[..16].copy_from_slice(&AsRef::<[f32; 16]>::as_ref(&proj)[..]);
        raw[16..32].copy_from_slice(&AsRef::<[f32; 16]>::as_ref(&proj_inv)[..]);
        raw[32..48].copy_from_slice(&AsRef::<[f32; 16]>::as_ref(&view)[..]);
        raw[48..51].copy_from_slice(AsRef::<[f32; 3]>::as_ref(&cam_pos));
        raw[51] = 1.0;
        raw
    }
}

struct State {
    camera: Camera,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize::<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    staging_belt: wgpu::util::StagingBelt,
    entities: Vec<Entity>,
    entity_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    depth_view: wgpu::TextureView,
}

impl State {

    const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;

    fn create_depth_texture(
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
    ) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        });
        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn configure_surface(&self) -> wgpu::SurfaceConfiguration {
       let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &config);
        return config;
    }

    async fn new(window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter= instance
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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            view_formats: vec![surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        surface.configure(&device, &config);

        let camera = Camera {
            screen_size: (config.width, config.height),
            angle_xz: 0.2,
            angle_y: 0.2,
            dist: 5.0,
        };

        let mut entities = Vec::new();
        {
            let mut vertices = Vec::<Vertex>::new();
            let poly = Polygon::<f32, Vec2f>::regular(Vec2f::new(0.0_f32, 0.0_f32), 1.0_f32, 6);

            let triangulation = poly.triangulate::<i32>();

            if triangulation.is_some() {

                let indices = triangulation.unwrap();
                let poly_vertices = poly.get_points();
                let poly_vertices_3f = embed_vertices_to_3f(CardinalDirection::Y, 0.0_f32, poly_vertices.to_vec());

                for i in indices {

                    let vertex: Vec3f = poly_vertices_3f[i as usize];
                    vertices.push(Vertex {
                        pos: [vertex.x(), vertex.y(), vertex.z()]
                    });

                }
            }
            
            let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            entities.push(Entity {
                vertex_count: vertices.len() as u32,
                vertex_buf,
            });

        }


        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Create the render pipeline
        let shader = device.create_shader_module(wgpu::include_wgsl!("polygon_shader.wgsl"));


        let raw_uniforms = camera.to_uniform_data();
        let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer"),
            contents: bytemuck::cast_slice(&raw_uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let polygon_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Polygon"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(config.view_formats[0].into())],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Self::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buf.as_entire_binding(),
                }
            ],
            label: None,
        });

        let depth_view = Self::create_depth_texture(&config, &device);


        let state = State {
            camera,
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            entities,
            staging_belt: wgpu::util::StagingBelt::new(0x100),
            entity_pipeline: polygon_pipeline,
            bind_group,
            uniform_buf,
            depth_view
        };

        state
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    #[expect(clippy::single_match)]
    fn update(&mut self, event: winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                let norm_x = position.x as f32 / self.camera.screen_size.0 as f32 - 0.5;
                let norm_y = position.y as f32 / self.camera.screen_size.1 as f32 - 0.5;
                self.camera.angle_y = norm_x * 5.0;
                self.camera.angle_xz = norm_y;
            }
            winit::event::WindowEvent::MouseWheel { device_id: _ , delta,.. } => {
                let mut dist = self.camera.dist;
                match delta {
                    MouseScrollDelta::PixelDelta (x) => {
                        let d = x.y as f32;
                        dist -= d;
                        dist = clamp(dist, 3.0, 20.0);
                    }
                    MouseScrollDelta::LineDelta(_x, y) => {
                        let d = y;
                        dist -= d;
                        dist = clamp(dist, 3.0, 20.0);
                    }
                }
                self.camera.dist = dist;
            }
            _ => {}
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        let config = self.configure_surface();
 
        self.depth_view = Self::create_depth_texture(&config, &self.device);
        self.camera.screen_size = (new_size.width, new_size.height);
    }

    fn render(&mut self) {

        let surface_texture =self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swapchain texture!");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
        });

        let mut encoder = self.device.create_command_encoder(&Default::default());

        let raw_uniforms = self.camera.to_uniform_data();
        self.staging_belt
            .write_buffer(
                &mut encoder,
                &self.uniform_buf,
                0,
                wgpu::BufferSize::new((raw_uniforms.len() * 4) as wgpu::BufferAddress).unwrap(),
                &self.device
            )
            .copy_from_slice(bytemuck::cast_slice(&raw_uniforms));

        self.staging_belt.finish();

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_pipeline(&self.entity_pipeline);

            for entity in self.entities.iter() {
                rpass.set_vertex_buffer(0, entity.vertex_buf.slice(..));
                rpass.draw(0..entity.vertex_count, 0..1);
            }

            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.window.pre_present_notify();
        surface_texture.present();

        self.staging_belt.recall();
    
    }
}

#[derive(Default)]
struct App {
    state: Option<State>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Polygon"))
                .unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
                // Emits a new redraw requested event.
                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                state.resize(size);
            }
            _ => (),
        }
        state.update(event);
    }
}

pub fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}