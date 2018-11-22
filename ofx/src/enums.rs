use ofx_sys::*;

pub trait IdentifiedEnum: Sized {
	fn to_bytes(&self) -> &'static [u8];
	fn from_bytes(ofx_name: &'static [u8]) -> Option<Self>;
}

macro_rules! identified_enum {{
		pub enum $name:ident {
			$($key:ident => $value:ident),*
		}
	} => {
		#[derive(Copy, Clone)]
		pub enum $name {
			$($key),
			*
		}

		impl IdentifiedEnum for $name {
			fn to_bytes(&self) -> &'static [u8] {
				match *self {
					$($name::$key => $value),
					*
				}
			}

			fn from_bytes(ofx_name: &'static [u8]) -> Option<Self> {
				$(if ofx_name == $value {
					Some($name::$key)
				} else)
				*
				{
					None
				}
			}
		}
	}
}

identified_enum! {
	pub enum ImageEffectContext {
		Filter => kOfxImageEffectContextFilter,
		General => kOfxImageEffectContextGeneral
	}
}

identified_enum! {
	pub enum BitDepth {
		Byte => kOfxBitDepthByte,
		Short => kOfxBitDepthShort,
		Float => kOfxBitDepthFloat
	}
}
