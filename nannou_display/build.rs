extern crate cc;

fn main() {
    cc::Build::new()
        .cuda(true)
        .flag("-lcuda")
        .flag("-lcudart")
        .flag("-gencode")
        .flag("arch=compute_86,code=sm_86")
        // .file("cuda/ray_tracer.cu")
        .compile("libcuda_helper.a");
}
