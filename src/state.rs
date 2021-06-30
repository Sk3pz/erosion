use winit::window::Window;
use winit::event::WindowEvent;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        /*
        The instance is a handle to our GPU
        BackendBit::
          PRIMARY        => All the apis that wgpu offers first teir of support for
                           -> Vulkan + Metal + DX12 + Browser WebGPU
          SECONDARY      => All the apis that wgpu offers second tier of support for. These may be unsupported/still experimental.
                           -> OpenGL + DX11
          VULKAN         => Supported on Windows, Linux/Android, and macOS/iOS via Vulkan Portability (with the Vulkan feature enabled)
          DX12           => Supported on Windows 10
          DX11           => Supported on Windows 7+
          BROWSER_WEBGPU => Supported when targeting the web through webassembly
          METAL          => Supported on macOS/iOS
          GL             => Unsupported
        */
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        // The surface is to create the swap_chain
        let surface = unsafe { instance.create_surface(window) };

        // The adapter is to create the device and queue
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface)
            }
        ).await.expect("Failed to create adapter for the surface.");

        // The device and the queue
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                // List of usable features here >> https://docs.rs/wgpu/0.7.0/wgpu/struct.Features.html
                features: wgpu::Features::empty(),
                // The limits of certain types of resources. Using defaults to support most platforms
                limits: wgpu::Limits::default(),
                label: None,
            },
            None // Trace path
        ).await.expect("Failed to create the device and the queue.");

        let sc_desc = wgpu::SwapChainDescriptor {
            // How the underlying textures will be used by the swapchain
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            // how the swapchain's textures will be stored on the GPU
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            // width and height of the swapchain in pixels
            width: size.width,
            height: size.height,
            // How to sync the swapchain with the display
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
        }
    }

    // Adds support for resizing the application
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // set the size to the new size
        self.size = new_size;
        // change the size of the swapchain descriptor
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        // re-create the swap_chain based on the swapchain descriptor
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    /// returns a bool to indicate whether an event has been fully processed
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                // TODO: Handle mouse movements
                true
            }
            _ => false
        }
    }

    pub fn update(&mut self) {
        // TODO: populate
    }


    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        // get a frame to render to
        let frame = self
            .swap_chain
            .get_current_frame()?
            .output;

        // create a command encoder to send commands to the gpu
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder")
            }
        );

        let render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0
                            }),
                            store: true
                        }
                    }
                ],
                depth_stencil_attachment: None,
            }
        );

        // release mutable borrow so that encoder.finish() can be called.
        drop(render_pass);

        // send the command buffer to the gpu's render queue
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}