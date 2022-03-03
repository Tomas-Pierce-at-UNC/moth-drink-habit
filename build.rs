
fn main() {
    cc::Build::new()
      .file("phantom/cineheaders.c")
      .file("phantom/cineutil.c")
      .file("phantom/cineimage.c")
      .include("phantom")
      .compile("cine");
}


