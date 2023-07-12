use image::{DynamicImage, GenericImageView};
use wgpu::{Device, TextureDescriptor};

pub struct Texture {
    pub image: DynamicImage,
    pub dimensions: (u32, u32),
    pub size: wgpu::Extent3d,
    pub inner_texture: wgpu::Texture,
}

impl Texture {
    pub fn new(device: &Device, file_path: &str) -> Self {
        let image = image::open(file_path).unwrap();
        let dimensions = image.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        Texture {
            image,
            dimensions,
            size,
            inner_texture: device.create_texture(&TextureDescriptor {
                label: Some("Diffuse Texture"),
                size: size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            }),
        }
    }

    pub fn get_image_copy(&self) -> wgpu::ImageCopyTexture {
        wgpu::ImageCopyTextureBase {
            texture: &self.inner_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        }
    }

    pub fn get_image_data_layout(&self) -> wgpu::ImageDataLayout {
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * self.dimensions.0),
            rows_per_image: Some(self.dimensions.1),
        }
    }

    pub fn get_bind_group_layout(&self, device: &Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture bind group layout descriptpr"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }
}
