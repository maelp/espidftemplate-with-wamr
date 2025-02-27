#[link(wasm_import_module = "host")]
extern "C" {
    fn extra() -> u32;
}

#[export_name = "add"]
pub fn add(m: u32, n: u32) -> u32 {
    m + n + unsafe { extra() }
}
