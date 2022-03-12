use windows::Win32::System::Com::{
    CoInitializeEx, CoUninitialize, COINIT, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
    COINIT_MULTITHREADED, COINIT_SPEED_OVER_MEMORY,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Flags(COINIT);

pub const APARTMENTTHREADED: Flags = Flags(COINIT_APARTMENTTHREADED);
pub const MULTITHREADED: Flags = Flags(COINIT_MULTITHREADED);
pub const DISABLE_OLE1DDE: Flags = Flags(COINIT_DISABLE_OLE1DDE);
pub const SPEED_OVER_MEMORY: Flags = Flags(COINIT_SPEED_OVER_MEMORY);

impl std::ops::BitOr for Flags {
    type Output = Self;
    
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

pub struct CoInitializer {
    _dummy: std::marker::PhantomData<()>,
}

#[inline]
pub fn init(flags: Flags) -> windows::core::Result<CoInitializer> {
    unsafe {
        CoInitializeEx(std::ptr::null(), flags.0)?;
    }
    Ok(CoInitializer {
        _dummy: std::marker::PhantomData,
    })
}

impl Drop for CoInitializer {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_test() {
        init(MULTITHREADED | DISABLE_OLE1DDE).unwrap();
    }
}