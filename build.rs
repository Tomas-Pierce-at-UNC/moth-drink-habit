
fn main() {
    cc::Build::new()
      .file("phantom/cineheaders.c")
      .file("phantom/cineutil.c")
      .include("phantom")
      .compile("cine");
}


