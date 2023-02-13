vec2 pixelize(vec2 uv, float size) {
    return (floor(uv * size) + 0.5) / size;
}
