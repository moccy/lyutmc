use ultraviolet as uv;

const VERTICES: [uv::Vec3; 8] = [
    uv::Vec3::new(-1.0, -1.0, 1.0),
    uv::Vec3::new(1.0, -1.0, 1.0),
    uv::Vec3::new(1.0, 1.0, 1.0),
    uv::Vec3::new(-1.0, 1.0, 1.0),
    uv::Vec3::new(-1.0, -1.0, -1.0),
    uv::Vec3::new(1.0, -1.0, -1.0),
    uv::Vec3::new(1.0, 1.0, -1.0),
    uv::Vec3::new(-1.0, 1.0, -1.0),
];

const INDICES: [u8; 36] = [
    0, 1, 2, 2, 3, 0, // front
    1, 5, 6, 6, 2, 1, // right
    7, 6, 5, 5, 4, 7, // back
    4, 0, 3, 3, 7, 4, // left
    4, 5, 1, 1, 0, 4, // bottom
    3, 2, 6, 6, 7, 3, // top
];
