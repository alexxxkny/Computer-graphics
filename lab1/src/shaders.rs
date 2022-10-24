pub const VERTEX_SHADER: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;
    uniform vec3 rotation_angles;      

    mat4 rotation3dX(float angle) {
        float s = sin(angle);
        float c = cos(angle);
    
        return mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, c, s, 0.0,
        0.0, -s, c, 0.0,
        0.0, 0.0, 0.0, 1.0
        );
    }

    mat4 rotation3dY(float angle) {
        float s = sin(angle);
        float c = cos(angle);
    
        return mat4(
        c, 0.0, -s, 0.0,
        0.0, 1.0, 0.0, 0.0,
        s, 0.0, c, 0.0,
        0.0, 0.0, 0.0, 1.0
        );
    }

    mat4 rotation3dZ(float angle) {
        float s = sin(angle);
        float c = cos(angle);
    
        return mat4(
        c, s, 0.0, 0.0,
        -s, c, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
        );
    }

    void main() {
        mat4 modelview = view * model;
        mat4 rotation = rotation3dX(rotation_angles.x) 
            * rotation3dY(rotation_angles.y) 
            * rotation3dZ(rotation_angles.z);
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * rotation * vec4(position, 1.0);
    }
"#;

pub mod pixel_shaders {
    pub const ROYAL_BLUE: &str = r#"
        #version 150

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.0, 0.6, 0.8);
            vec3 regular_color = vec3(0.2, 0.9, 0.8);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;
}