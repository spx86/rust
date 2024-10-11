mod abi;
pub use abi::*;

use base64::{engine::general_purpose, Engine};
use photon_rs::transform::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

impl From<&ImageSpec> for String {
    fn from(spec: &ImageSpec) -> Self {
        let data = spec.encode_to_vec();
        general_purpose::URL_SAFE_NO_PAD.encode(&data)
    }
}

impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = general_purpose::URL_SAFE_NO_PAD.decode(value).unwrap();
        Ok(ImageSpec::decode(&data[..])?)
    }
}

impl filter::Filter{
    pub fn to_str(&self) -> Option<&'static str>{
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Island => Some("island"),
            filter::Filter::Marine => Some("marine"),
            filter::Filter::Oceanic => Some("oceanic"),
        }
    }
}

impl From<resize::SampleFilter> for SamplingFilter{
    fn from(value: resize::SampleFilter) -> Self {
        match value {
            resize::SampleFilter::Undefined => SamplingFilter::Nearest,
            resize::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize::SampleFilter::Triangle => SamplingFilter::Triangle,
            resize::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            resize::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize::SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize{
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32, filter:resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize{
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }

    pub fn new_filter( filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter{
                filter: filter as i32,
            })),
        }
    }

    pub fn new_watermark(x:u32, y:u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark{
                x,
                y,
            })),
        }
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;
    use std::convert::TryInto;

    #[test]
    fn encoded_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }
}