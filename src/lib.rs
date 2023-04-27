//! FRUG is intended to provide a similar abstraction layer over graphics programming as to how SDL does for C++, meaning that it should provide developers enough control and flexibility to implement their own architectures & design patterns, yet simplifying the process of working with graphics so developers won't have to worry about implementing all the repetitive tasks related to getting things to the screen.
//! 
//! FRUG aims to include the following features (unchecked items are the ones still under development):
//! - [x] Window management
//! - [ ]  Loading & rendering textures
//! - [ ]  Rotating textures
//! - [ ]  Scaling textures
//! - [ ]  Alpha blending for textures
//! - [ ]  Choosing a specific backend (aka. Direct X, Metal, Vulkan, etc.)
//! - [ ]  Writing and using custom shaders
//! - [ ]  Handle window state events
//! - [ ]  Handle Mouse input
//! - [ ]  Handle Keyboard input
//! - [ ]  Playing audio
//! - [ ]  Configure audio


// Exported libs
pub use winit::event::{VirtualKeyCode, KeyboardInput};
pub use winit_input_helper::WinitInputHelper as InputHelper;

// Internal use
use wgpu::{util::DeviceExt};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::Window
};

mod texture;

pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other
}

impl Into<usize> for MouseButton {
    fn into(self) -> usize {
        self as usize
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0, 
    0.0, 0.0, 0.5, 1.0
);

struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(
            cgmath::Deg(self.fovy), 
            self.aspect, 
            self.znear, 
            self.zfar
        );

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

/// Our camera uniform to store the view projection matrix.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4]
}

impl CameraUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self { view_proj: cgmath::Matrix4::identity().into() }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

/// Vertex struct
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub text_coords: [f32; 2]
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex { 
            position: [0.0, 0.0, 0.0], 
            text_coords: [0.0, 0.0]
        }
    }
}

/// Implementation of Vertex methods
impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout { 
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3
                },
                // text coords
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2
                },
                // color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3
                }
            ] 
        }
    }
}

/// Textured Object struct
/// Contains:
/// `n_indices (u8)`        - The number of indices the object holds.
/// `bind_group_idx (u32)`  - The index of the bind group to use.
struct TexturedObj {
    indices_low_pos: u32,
    indices_hi_pos: u32,
    bind_group_idx: usize
}

/// The Frug instance.
/// Contains the surface in which we draw, the device we're using, the queue, the surface configuration, surface size, window, background color, and render pipeline.
pub struct FrugInstance {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    background_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    staging_vertices: Vec<Vertex>,
    staging_indices: Vec<u16>,
    num_indices: u32,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    diffuse_bind_groups: Vec<wgpu::BindGroup>,
    textured_objects: Vec<TexturedObj>,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup
}

/// Implementation of FrugInstance methods
impl FrugInstance {
    /// Creates a new instance of FrugInstance, instantiating the window, configuration, and the surface to draw in.
    async fn new_instance(window_title: &str, event_loop: &EventLoop<()>) -> Self {
        // Enable wgpu logging
        env_logger::init();

        // Setup
        let window = Window::new(&event_loop).unwrap();
        window.set_title(window_title);
        let size = window.inner_size();
        let background_color = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
        let vertices: &[Vertex] = &[];
        let indices: &[u16] = &[];

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let surface = unsafe { 
            instance.create_surface(&window)
        }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.expect("Failed to find an appropiate adapter.");

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default()
            }, None).await.expect("Failed to create device.");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // Camera
        let camera = Camera {
            eye: (0.0, 0.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer { 
                            ty: wgpu::BufferBindingType::Uniform, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        },
                        count: None
                    }
                ],
                label: Some("Camera bind group layout")
            }
        );

        let camera_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding()
                    }
                ],
                label: Some("Camera bind group")
            }
        );

        // we use this to load textures
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
                label: Some("texture_bind_group_layout"), 
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture { 
                            sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                            view_dimension: wgpu::TextureViewDimension::D2, 
                            multisampled: false 
                        },
                        count: None
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None
                    }
                ]
            });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &texture_bind_group_layout,
                &camera_bind_group_layout
            ],
            push_constant_ranges: &[]
        });

        // our render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState { 
                module: &shader, 
                entry_point: "vs_main", 
                buffers: &[Vertex::desc()] 
            },
            fragment: Some(wgpu::FragmentState { 
                module: &shader, 
                entry_point: "fs_main", 
                targets: &[Some(wgpu::ColorTargetState { 
                    format: config.format, 
                    blend: Some(wgpu::BlendState::REPLACE), 
                    write_mask: wgpu::ColorWrites::ALL 
                })]
            }),
            primitive: wgpu::PrimitiveState { 
                topology: wgpu::PrimitiveTopology::TriangleList, 
                strip_index_format: None, 
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back), 
                unclipped_depth: false, 
                polygon_mode: wgpu::PolygonMode::Fill, 
                conservative: false 
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState { 
                count: 1, 
                mask: !0, 
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX
        });

        let num_indices = indices.len() as u32;

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            background_color,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            staging_vertices: Vec::new(),
            staging_indices: Vec::new(),
            num_indices,
            texture_bind_group_layout,
            diffuse_bind_groups: Vec::new(),
            textured_objects: Vec::new(),
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group
        }
    }

    /// Resize the canvas for our window given a new defined size.
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Renders all textured objects based on data on buffers.
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        // draw our objects
        let mut render_pass_op = wgpu::LoadOp::Clear(self.background_color);
        for tex_obj in &self.textured_objects {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                label: Some("Render Pass"), 
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations { 
                        load: render_pass_op, 
                        store: true
                    }
                })], 
                depth_stencil_attachment: None
            });

            render_pass.set_pipeline(&self.render_pipeline);

            // texture bind group
            render_pass.set_bind_group(
                0, 
                &self.diffuse_bind_groups[tex_obj.bind_group_idx], 
                &[]
            );

            // camera bind group
            render_pass.set_bind_group(
                1, 
                &self.camera_bind_group, 
                &[]
            );

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(tex_obj.indices_low_pos..tex_obj.indices_hi_pos, 0, 0..1);
            render_pass_op = wgpu::LoadOp::Load;
        }

        // Clear objects
        self.textured_objects.clear();

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn update(&mut self) {
        //todo!();
    }

    /// Sets new background color.
    /// 
    /// Receives a wgpu color (you can create one using the `frug::create_color` method).
    /// 
    /// # Example
    /// ```
    /// let new_color = frug::create_color(0.2, 0.3, 0.4, 1.0);
    /// my_frug_instance.set_background_color(new_color);
    /// ```
    pub fn set_background_color(&mut self, color: wgpu::Color) {
        self.background_color = color;
    }

    /// Updates the vertex and index buffers with the staging data.
    pub fn update_buffers(&mut self) {
        self.vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.staging_vertices),
            usage: wgpu::BufferUsages::VERTEX
        });

        self.index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.staging_indices),
            usage: wgpu::BufferUsages::INDEX
        });

        self.num_indices = self.staging_indices.len() as u32;
    }

    /// Adds a set of vertices and indices to the staging data.
    pub fn add_staging_indexed_vertices(&mut self, vertices: &[Vertex], indices: &[u16]) {

        // update the indices to match the number of current vertices
        let offset: u16 = self.staging_vertices.len() as u16;
        for index in indices {
            self.staging_indices.push(index + offset);
        }

        self.staging_vertices.extend(vertices);
    }

    /// Clears the staging buffers data so the next frame is empty.
    pub fn clear(&mut self) {
        self.staging_vertices.clear();
        self.staging_indices.clear();
    }

    /// Adds a rectangle to the staging data using a texture.
    /// Receives:
    /// * `x (f32)`             - The x origin of the rectangle.
    /// * `y (f32)`             - The y origin of the rectangle.
    /// * `w (f32)`             - The width of the rectangle.
    /// * `h (f32)`             - The height of the rectangle.
    /// * `texture_index (u16)` - The index of the texture we're drawing.
    pub fn add_tex_rect(&mut self, x: f32, y: f32, w: f32, h: f32, texture_index: usize) {

        // Add the object to the textured objects vector
        let low_bound = self.staging_indices.len() as u32;
        self.textured_objects.push(TexturedObj { 
            indices_low_pos: low_bound,
            indices_hi_pos: low_bound + 6,
            bind_group_idx: texture_index 
        });

        // TODO: We should update these text_coords to match the actual coordinates.
        //      NOTE: Maybe this is correct as it is.
        // TODO: We should be able to choose which texture this rect is using.
        self.add_staging_indexed_vertices(
            &[
            Vertex { position: [x, y, 0.0], text_coords: [0.0, 0.0] },
            Vertex { position: [x, y-h, 0.0], text_coords: [0.0, 1.0] },
            Vertex { position: [x+w, y-h, 0.0], text_coords: [1.0, 1.0] },
            Vertex { position: [x+w, y, 0.0], text_coords: [1.0, 0.0] },
        ], &[
            0, 1, 3,
            1, 2, 3,
        ]);
    }

    /// Adds a rectangle to the staging data using a texture.
    /// Receives:
    /// * `x (f32)`             - The x origin of the rectangle.
    /// * `y (f32)`             - The y origin of the rectangle.
    /// * `w (f32)`             - The width of the rectangle.
    /// * `h (f32)`             - The height of the rectangle.
    /// * `color [f32; 3]`      - The [red, green, blue] definition of the color to use.
    pub fn add_colored_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 3]) {

    }
    /// Loads a texture
    pub fn load_texture(&mut self, img_bytes: &[u8]) -> usize {
        
        let diffuse_texture = texture::Texture::from_bytes(&self.device, &self.queue, img_bytes, "texture").unwrap();


        let diffuse_bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor { 
                label: Some("diffuse_bind_group"), 
                layout: &self.texture_bind_group_layout, 
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view)
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler)
                    }
                ]
            }
        );

        self.diffuse_bind_groups.push(diffuse_bind_group);

        return self.diffuse_bind_groups.len() - 1;

    }

    /// Starts running the loop
    pub fn run<F: 'static + FnMut(&mut FrugInstance, &winit_input_helper::WinitInputHelper)>(mut self, event_loop: EventLoop<()>, mut update_function: F) {

        let mut input = winit_input_helper::WinitInputHelper::new();

        // Run the loop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            input.update(&event);
            
            // Act on events
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } 
                // Window events
                if window_id == self.window.id() => match event {
                    // Close
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    },
                    
                    // Resize
                    WindowEvent::Resized(physical_size) => {
                        self.resize(*physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.resize(**new_inner_size);
                    }
                    
                    _ => ()
                }
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    self.update();
                    match self.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => self.resize(self.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();

                    update_function(&mut self, &input);
                }
                _ => (),
            }

            
        });
    }
}

/// Inits your frug instance and your event loop.
/// Returns a pair containing a FrugInstance and an EventLoop.
pub fn new(window_title: &str) -> (FrugInstance, EventLoop<()>) {
    let event_loop = EventLoop::new();
    let frug_instance = pollster::block_on( FrugInstance::new_instance(window_title, &event_loop));

    return (frug_instance, event_loop);
}

/// Creates a color.
/// Should receive in range from 0.0 - 1.0 the red, green, blue, and alpha channels.
/// * `red (f64)`   - The red channel.
/// * `green (f64)`   - The green channel.
/// * `blue (f64)`   - The blue channel.
/// * `alpha (f64)`   - The alpha channel.
/// 
/// # Example:
/// 
/// ```
/// frug::create_color(0.1, 0.2, 0.3, 1.0);
/// ```
pub fn create_color(red: f64, green: f64, blue: f64, alpha: f64) -> wgpu::Color {
    wgpu::Color { r: red, g: green, b: blue, a: alpha }
}

// EOF