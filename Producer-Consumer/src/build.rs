use cc;

fn main() {
    cc::Build::new().file("src/tands.c").compile("libtands.a")
}
