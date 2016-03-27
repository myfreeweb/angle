extern crate libc;

pub mod ffi;
pub mod hl;

#[cfg(test)]
mod tests {
    use ffi::*;
    use hl::*;
    use std::sync::{ONCE_INIT, Once};
    static GLSLANG_INITIALIZATION: Once = ONCE_INIT;

    fn init() {
        GLSLANG_INITIALIZATION.call_once(|| initialize().unwrap());
    }


    #[test]
    fn test_linkage() {
        init();
    }

    #[test]
    fn test_translation() {
        const SHADER: &'static str = "void main() {
    gl_FragColor = vec4(0, 1, 0, 1);  // green
}";
        const EXPECTED: &'static str = "void main(){
(gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0));
}\n";
        const FRAGMENT_SHADER: u32 = 0x8B30;

        init();

        let compiler = ShaderValidator::for_webgl(FRAGMENT_SHADER,
                                                  Output::Glsl,
                                                  &BuiltInResources::default()).unwrap();

        let result = compiler.compile_and_translate(&[SHADER.to_owned().as_bytes()]).unwrap();
        println!("{:?}", result);
        assert!(result == EXPECTED);
    }

    // TODO(emilio): run this test. We can't actually run it because travis machines can't output
    // essl.
    fn test_translation_essl() {
        const SHADER: &'static str = "void main() {
    gl_FragColor = vec4(0, 1, 0, 1);  // green
}";
        const EXPECTED: &'static str = "void main(){
(gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0));
}\n";
        const FRAGMENT_SHADER: u32 = 0x8B30;

        init();

        let compiler = ShaderValidator::for_webgl(FRAGMENT_SHADER,
                                                  Output::Essl,
                                                  &BuiltInResources::default()).expect("Failed to create a validator for essl");

        let result = compiler.compile_and_translate(&[SHADER.to_owned().as_bytes()]).unwrap();
        println!("{:?}", result);
        assert!(result == EXPECTED);
    }
}
