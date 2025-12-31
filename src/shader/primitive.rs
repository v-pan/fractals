use crate::shader::pipeline::ComputeShaderPipeline;
use crate::shader::pipeline::RenderShaderPipeline;
use iced::widget::shader::wgpu;
use iced::{
    widget::shader::{self},
    Rectangle,
};

#[derive(Debug)]
pub struct ShaderPrimitive {}

impl ShaderPrimitive {
    pub fn new() -> Self {
        Self {}
    }
}

impl shader::Primitive for ShaderPrimitive {
    fn prepare(
        &self,
        format: shader::wgpu::TextureFormat,
        device: &shader::wgpu::Device,
        queue: &shader::wgpu::Queue,
        _bounds: iced::Rectangle,
        target_size: iced::Size<u32>,
        _scale_factor: f32,
        storage: &mut shader::Storage,
    ) {
        if !storage.has::<ComputeShaderPipeline>() {
            storage.store(ComputeShaderPipeline::new(device, target_size));
        }
        if !storage.has::<RenderShaderPipeline>() {
            storage.store(RenderShaderPipeline::new(device, format, target_size));
        }

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("compute command encoder"),
        });

        let pipeline = storage.get_mut::<ComputeShaderPipeline>().unwrap();

        // TODO: Add uniforms
        //
        // pipeline.update();

        pipeline.dispatch(&mut encoder);
        queue.submit(Some(encoder.finish()));

        // Debug
        //
        // let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        //     label: None,
        //     size: (target_size.width * target_size.height * 4) as u64,
        //     usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        //     mapped_at_creation: false,
        // });
        // let bytes_per_row =
        //     (4 * target_size.width).next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
        // encoder.copy_texture_to_buffer(
        //     pipeline.screen_texture.as_image_copy(),
        //     wgpu::ImageCopyBuffer {
        //         buffer: &buffer,
        //         layout: wgpu::ImageDataLayout {
        //             offset: 0,
        //             bytes_per_row: Some(bytes_per_row),
        //             rows_per_image: Some(target_size.height),
        //         },
        //     },
        //     pipeline.screen_texture.size(),
        // );
        // queue.submit(Some(encoder.finish()));
        // buffer.slice(..).map_async(wgpu::MapMode::Read, |result| {});
        // device.poll(wgpu::Maintain::Wait);
        // let data = buffer.slice(..).get_mapped_range();
        // println!("First pixel: {:?}", &data[0..4]); // Should be [255,0,0,255] for red
    }

    fn render(
        &self,
        storage: &shader::Storage,
        target: &shader::wgpu::TextureView,
        _target_size: iced::Size<u32>,
        viewport: Rectangle<u32>,
        encoder: &mut shader::wgpu::CommandEncoder,
    ) {
        let compute_pipeline = storage.get::<ComputeShaderPipeline>().unwrap();
        let render_pipeline = storage.get::<RenderShaderPipeline>().unwrap();
        compute_pipeline.dispatch(encoder);
        render_pipeline.render(encoder, target, &compute_pipeline.screen_texture);
    }
}
