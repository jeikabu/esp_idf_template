#![no_std]
#![no_main]

use esp_idf_sys as idf;

#[no_mangle]
pub fn app_main() {
    esp_idf_logger::init().unwrap();
    unsafe {
        
    }
}

extern "C" {
    fn abort() -> !;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        abort();
    }
}
