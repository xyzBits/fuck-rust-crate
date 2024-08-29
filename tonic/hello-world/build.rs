use std::fs;

// 编译时，会优先 执行 build rs cargo 编译体系的规范
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("src/pb")?;

    tonic_build::configure()
        .out_dir("src/pb")
        .build_server(true)
        .build_client(true)
        // include 是指所依赖的文件需要在哪个目录中找
        .compile(&["proto/hello.proto"], &["protos"])?;

    Ok(())
}
