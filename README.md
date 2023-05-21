<div align="center">
  <img src="https://vonbrank-images.oss-cn-hangzhou.aliyuncs.com/20230429-Ray-Tracing-Rust/ray-tracing-the-next-week-final-540p-spp5000-01.jpg" alt="Banner image" style="max-width: 100%; object-fit: cover; width: 480px;">
  
  <h3 align="center">Basic Ray Tracer in Rust</h3>

  <p align="center">
    使用 Rust 实现的基础光线追踪渲染器
  </p>
</div>

## 关于本项目

本项目参考 [Ray Tracing in One Weekend](https://raytracing.github.io/) 系列教程，使用 Rust 语言实现了基础的递归式光线追踪算法。

目前已完成前两阶段：
+ [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
  
<div align="center">
  <img src="https://vonbrank-images.oss-cn-hangzhou.aliyuncs.com/20230429-Ray-Tracing-Rust/image-1080p-spp200-01.jpg" alt="Banner image" style="max-width: 100%; object-fit: cover; width: 480px;">
</div>

+ [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html)

<div align="center">
  <img src="https://vonbrank-images.oss-cn-hangzhou.aliyuncs.com/20230429-Ray-Tracing-Rust/ray-tracing-the-next-week-final-540p-spp5000-01.jpg" alt="Banner image" style="max-width: 100%; object-fit: cover; width: 480px;">
</div>

依据教程实现了以下特性：
+ Ray Tracing in One Weekend:
  + 球体 Mesh
  + 随机采样抗锯齿：在每个像素内随机采样取平均值，减少噪点的同时实现抗锯齿。
  + 材质：包括 Diffuse、Glossy、镜面、半透明 等材质。
  + 模拟景深
+ Ray Tracing: The Next Week:
  + 模拟运动模糊
  + 层次包围盒 BVH
  + 柏林噪声
  + 矩形 Fragment、长方体 Mesh
  + 发光材质
  + 平移、旋转变换
  + 体积光

此外，项目还实现了这些特性：
+ 多线程渲染：为了提升性能，减少单次渲染消耗的时间，我们实现了一个线程池。渲染开始前，程序将自动检测设备的逻辑线程数，以确定线程池中的最大可用线程数。每一行像素的渲染将会被分配到不同线程去执行。
+ 可视化渲染进度：渲染开始后，可以从终端实时查看当前渲染进度和已渲染像素数等信息：
  ```txt
  Rendering:  [###----------------------] 12%
  00:00:30    7502 pixel(s) per second
  threads=16  253844 pixel(s) rendered
  ```

## 使用方法

+ 安装 `rustup`

+ 运行

  ```bash
  cargo run --release > ${图片路径/名称.ppm}
  ```

  光线追踪算法对性能需求非常高，因此请确保使用 `--release` 选项运行程序，以释放 Rust 的全部性能。

  本项目保存图片的格式为 `.ppm` ，与原教程相同。

+ 查看输出结果
  
  您可以使用任何支持 `.ppm` 格式的图片查看器查看输出结果，例如 [这款 VSCode 插件](https://marketplace.visualstudio.com/items?itemName=martingrzzler.simple-ppm-viewer) 或 [这款 PPM 在线查看器](https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html)
