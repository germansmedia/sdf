// SDF - render fractals in VR
// by Desmond Germans, 2023

use {
    crate::*,
    std::{
        result::Result,
        sync::Arc,
        mem::size_of,
    },
};

#[derive(Clone,Copy,Debug)]
#[repr(C)]
struct Uniforms {
    matrix: Mat4x4<f32>,
    fovs: [Fov<f32>; 2],
}

#[derive(Vertex)]
struct FlatVertex {
    _pos: Vec2<f32>,
}

struct Context {
    command_buffer: Arc<CommandBuffer>,
    _framebuffer: Arc<Framebuffer>,
}

pub struct Projector {
    app: Arc<App>,
    _gpu: Arc<Gpu>,
    queue: Arc<Queue>,
    main_view: Arc<View>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<Image2D>>,
    layers: usize,
    _rgba_image: Arc<Image2D>,
    _generic_quad: Arc<VertexBuffer>,
    _render_pass: Arc<RenderPass>,
    _descriptor_set_layout: Arc<DescriptorSetLayout>,
    _uniforms: Uniforms,
    uniform_buffer: Arc<UniformBuffer>,
    _sampler: Arc<Sampler>,
    _descriptor_sets: Vec<Arc<DescriptorSet>>,
    _pipeline_layout: Arc<PipelineLayout>,
    _vertex_shader: Arc<VertexShader>,
    _fragment_shader: Arc<FragmentShader>,
    _graphics_pipeline: Arc<GraphicsPipeline>,
    contexts: Vec<Vec<Context>>,
    orientation: Quat<f32>,
}

impl Projector {

    pub fn new(app: &Arc<App>,rgba_image: &Arc<Image2D>) -> Result<Projector,String> {

        let gpu = app.gpu();
        let queue = gpu.queue(0)?;
        let main_view = app.main_view();
        let swapchain = main_view.swapchain();
        let images = swapchain.images();
        let layers = swapchain.layers();

        // create generic quad
        let mut vertices = Vec::<FlatVertex>::new();
        vertices.push(FlatVertex { _pos: Vec2 { x: -1.0,y: -1.0, }, });
        vertices.push(FlatVertex { _pos: Vec2 { x: 1.0,y: -1.0, }, });
        vertices.push(FlatVertex { _pos: Vec2 { x: 1.0,y: 1.0, }, });
        vertices.push(FlatVertex { _pos: Vec2 { x: -1.0,y: 1.0, }, });
        let generic_quad = gpu.create_vertex_buffer(Init::Data(&vertices))?;

        // create render pass to display rgba_image around user
        let render_pass = gpu.create_color_render_pass(ImageFormat::RGBA8SRGB,1)?;
        let descriptor_set_layout = gpu.build_descriptor_set_layout()
            .uniform_buffer()
            .sampled_image2dview()
            .build()?;
        let mut rgba_image_views = Vec::<Arc<Image2DView>>::new();
        for i in 0..layers {
            rgba_image_views.push(rgba_image.create_view(i,0,1)?);
        }
        let uniforms = Uniforms {
            matrix: Mat4x4::ONE,
            fovs: [Fov { l: 0.0,r: 0.0,b: 0.0,t: 0.0, }; 2],
        };
        let uniform_buffer = gpu.create_uniform_buffer(Init::Data(&[uniforms]))?;
        let sampler = gpu.create_sampler(
            SamplerFilter::Linear,
            SamplerFilter::Linear,
            SamplerFilter::Linear,
            AddressMode::ClampToEdge,
            AddressMode::ClampToEdge,
            AddressMode::ClampToEdge,
            0.0,
            Anisotropy::Disabled,
            CompareOp::Always,
            0.0,
            0.0,
            Vec4::<f32> { x: 0.0,y: 0.0,z: 0.0,w: 0.0, },
            false,
        )?;
        let mut descriptor_sets = Vec::<Arc<DescriptorSet>>::new();
        for i in 0..layers {
            descriptor_sets.push(descriptor_set_layout.build_descriptor_set()?
                .uniform_buffer(&uniform_buffer)
                .sampled_image2dview(&rgba_image_views[i],&sampler)
                .build()
            );
        }
        let pipeline_layout = gpu.create_pipeline_layout(&[&descriptor_set_layout],size_of::<Push>())?;
        let code = app.load_asset("assets","draw_equirect_vs.spirv")?;
        let vertex_shader = gpu.create_vertex_shader(&code)?;
        let code = app.load_asset("assets","draw_equirect_fs.spirv")?;
        let fragment_shader = gpu.create_fragment_shader(&code)?;
        let graphics_pipeline = pipeline_layout.create_graphics_pipeline::<FlatVertex>(
            &render_pass,
            &vertex_shader,
            &fragment_shader,
            PrimitiveTopology::TriangleFan,
            PrimitiveRestart::Disabled,
            0,
            DepthClamp::Disabled,
            PrimitiveDiscard::Disabled,
            PolygonMode::Fill,
            CullMode::None,
            DepthBias::Disabled,
            1.0,
            1,
            SampleShading::Disabled,
            AlphaToCoverage::Enabled,
            AlphaToOne::Disabled,
            DepthTest::Disabled,
            true,
            StencilTest::Disabled,
            LogicOp::Disabled,
            Blend::Disabled,
            (true,true,true,true),
            Vec4 { x: 0.0,y: 0.0,z: 0.0,w: 1.0, },
        )?;

        // create command buffers for each swapchain image
        let mut contexts = Vec::<Vec<Context>>::new();
        for image in images.iter() {
            let size = image.size();
            let mut layer_contexts = Vec::<Context>::new();
            for i in 0..layers {
                let color_view = image.create_view(i,0,1)?;
                let framebuffer = render_pass.create_color_framebuffer(&color_view,size.into())?;
                let r = Rect::<isize> { o: Vec2::ZERO,s: size.into(), };
                let command_buffer = queue.create_command_buffer()?;
                command_buffer.begin()?;
                command_buffer.bind_graphics_pipeline(&graphics_pipeline);
                command_buffer.set_viewport(r,0.0,1.0);
                command_buffer.set_scissor(r);
                command_buffer.bind_graphics_descriptor_set(&pipeline_layout,0,&descriptor_sets[i]);
                command_buffer.bind_vertex_buffer(&generic_quad);
                command_buffer.push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Square,y_offset: 0, });
                command_buffer.begin_render_pass(&framebuffer,r);
                command_buffer.draw(4,1,0,0);
                command_buffer.end_render_pass();
                command_buffer.end()?;
                layer_contexts.push(Context {
                    command_buffer,
                    _framebuffer: framebuffer,
                });
            }
            contexts.push(layer_contexts);
        }

        Ok(Projector {
            app: Arc::clone(&app),
            _gpu: gpu,
            queue,
            main_view,
            swapchain,
            images,
            layers,
            _rgba_image: Arc::clone(&rgba_image),
            _generic_quad: generic_quad,
            _render_pass: render_pass,
            _descriptor_set_layout: descriptor_set_layout,
            _uniforms: uniforms,
            uniform_buffer,
            _sampler: sampler,
            _descriptor_sets: descriptor_sets,
            _pipeline_layout: pipeline_layout,
            _vertex_shader: vertex_shader,
            _fragment_shader: fragment_shader,
            _graphics_pipeline: graphics_pipeline,
            contexts,
            orientation: Quat::ONE,
        })
    }

    pub fn render(&mut self,t: u64) -> Result<(),String> {
        let index = self.swapchain.acquire()?;
        self.orientation = self.app.local_space().locate_other(&self.app.head_space(),t)?.o;
        let matrix = Mat4x4::<f32>::from(self.orientation).inv().transpose();
        self.uniform_buffer.data_mut()?[0] = Uniforms {
            matrix,
            fovs: [self.main_view.fov(0),self.main_view.fov(1)],
        };
        for _ in 0..self.layers {
            self.queue.transition_image2d_layout(&self.images[index],PipelineStage::ColorAttachmentOutput,PipelineStage::ColorAttachmentOutput,ImageLayout::General,0,1)?;
        }
        for i in 0..self.layers {
            self.queue.submit(&self.contexts[index][i].command_buffer,None,None)?;
        }
        for _ in 0..self.layers {
            self.queue.transition_image2d_layout(&self.images[index],PipelineStage::ColorAttachmentOutput,PipelineStage::ColorAttachmentOutput,ImageLayout::Present,0,1)?;
        }
        self.swapchain.release(index)
    }

    pub fn head_orientation(&self) -> Quat<f32> {
        self.orientation
    }
}
