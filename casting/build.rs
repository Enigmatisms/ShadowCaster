extern crate cc;

fn main() {
    cc::Build::new()
        .cuda(true)
        .flag("-lcuda")
        .flag("-lcudart")
        .flag("-gencode")
        .flag("arch=compute_86,code=sm_86")
        .file("cuda/cast_kernel.cu")
        .file("cuda/shadow_cast.cu")
        .compile("libcast_impl.a");
}
