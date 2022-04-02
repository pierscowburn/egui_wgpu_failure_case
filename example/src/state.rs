use log::info;
use winit::{event::Event, window::Window};

#[cfg(feature = "webgl")]
const BACKENDS: wgpu::Backends = wgpu::Backends::GL;
#[cfg(not(feature = "webgl"))]
const BACKENDS: wgpu::Backends = wgpu::Backends::BROWSER_WEBGPU;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    screen_descriptor: egui_wgpu_backend::ScreenDescriptor,
    performance: web_sys::Performance,
    platform: egui_winit_platform::Platform,
    egui_render_pass: egui_wgpu_backend::RenderPass,
    meshes: Option<Vec<egui::ClippedMesh>>,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(BACKENDS);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        info!("Adapter info: {:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        let format = surface.get_preferred_format(&adapter).unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
            physical_width: config.width,
            physical_height: config.height,
            scale_factor: window.scale_factor() as f32,
        };

        let performance = web_sys::window().unwrap().performance().unwrap();

        let platform =
            egui_winit_platform::Platform::new(egui_winit_platform::PlatformDescriptor {
                physical_width: size.width as u32,
                physical_height: size.height as u32,
                scale_factor: window.scale_factor(),
                font_definitions: egui::FontDefinitions::default(),
                style: Default::default(),
            });

        let egui_render_pass = egui_wgpu_backend::RenderPass::new(&device, format, 1);

        Self {
            surface,
            device,
            queue,
            screen_descriptor,
            performance,
            platform,
            egui_render_pass,
            meshes: None,
        }
    }

    pub fn update(&mut self, event: Event<()>, window: &Window) {
        self.platform.handle_event(&event);

        self.platform.update_time(self.performance.now() / 1000.0);

        self.platform.begin_frame();

        egui::CentralPanel::default().show(&self.platform.context(), |ui| {
            ui.painter().rect_filled(
                egui::Rect::from_center_size(
                    egui::Pos2::new(0.0, 0.0),
                    egui::Vec2::new(4000.0, 4000.0),
                ),
                0.0,
                egui::Color32::GREEN,
            );
        });

        let (_output, shapes) = self.platform.end_frame(Some(&window));

        self.meshes = Some(self.platform.context().tessellate(shapes));
    }

    pub fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        self.egui_render_pass.update_texture(
            &self.device,
            &self.queue,
            &self.platform.context().font_image(),
        );
        self.egui_render_pass
            .update_user_textures(&self.device, &self.queue);
        self.egui_render_pass.update_buffers(
            &self.device,
            &self.queue,
            self.meshes.as_ref().unwrap(),
            &self.screen_descriptor,
        );

        self.egui_render_pass
            .execute(
                &mut encoder,
                &view,
                self.meshes.as_ref().unwrap(),
                &self.screen_descriptor,
                Some(wgpu::Color::RED),
            )
            .unwrap();

        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();
    }
}
