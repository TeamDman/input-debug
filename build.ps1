$old = $env:RUSTFLAGS
$env:RUSTFLAGS = "-Zlocation-detail=none -Zfmt-debug=none"
try {
    cargo build `
    -Z build-std=std,panic_abort `
    -Z build-std-features="optimize_for_size" `
    -Z build-std=std,panic_abort `
    -Z build-std-features=panic_immediate_abort `
    --target x86_64-pc-windows-msvc `
    --release 
} finally {
    $env:RUSTFLAGS = $old
}