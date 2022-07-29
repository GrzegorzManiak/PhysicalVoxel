use bevy::{prelude::*, render::mesh::{PrimitiveTopology, Indices}};

use crate::components::{Chunk, MarchingCubeSystem, Triangle};
mod marching_cube_table;
use marching_cube_table::*;

fn u8_to_grayscale(u8_value: u8) -> Color {
    let value = u8_value as f32 / 255.0;
    Color::rgb(value, value, value)
}

pub fn new(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {

    // -- [ Point cloud generation ] -- //
    let mut chunk = Chunk::default();

    for x in 0..u32::pow(chunk.size + 1, 3) {
        // -- Generate a random number --
        let random_number = rand::random::<u8>();

        chunk.vertex_vector.push(50);
    }


    // -- [ Marching cube ] -- //
    let mut triangles = Vec::new();

    for cube in 0..u32::pow(chunk.size, 3) {
        let points = get_cube_points(&mut chunk, cube); 

        let cube_index = get_cube_index(&mut chunk, points);

        let corner_coordinates = corner_coordinates(&mut chunk, cube); 

        let edge_indices = TRIANGULATION_TABLE[cube_index as usize]; 

        // -- Create triangles for the current cube configuration
        for i in [0, 3, 6, 9, 12, 15].iter() {
            if edge_indices[*i as usize] == -1 { break; }

            let a0e = edge_indices[i + 0] as usize;
            let a0 = CORNER_INDEX[a0e][0];
            let a1 = CORNER_INDEX[a0e][1];

            let b0e = edge_indices[i + 1] as usize;
            let b0 = CORNER_INDEX[b0e][0];
            let b1 = CORNER_INDEX[b0e][1];

            let c0e = edge_indices[i + 2] as usize;
            let c0 = CORNER_INDEX[c0e][0];
            let c1 = CORNER_INDEX[c0e][1];

            let mut triangle: Triangle = Triangle::default();

            triangle.a = (corner_coordinates[a0 as usize] + corner_coordinates[a1 as usize]) * 0.5;
            triangle.b = (corner_coordinates[b0 as usize] + corner_coordinates[b1 as usize]) * 0.5;
            triangle.c = (corner_coordinates[c0 as usize] + corner_coordinates[c1 as usize]) * 0.5;

            triangles.push(triangle);
        }
    }

    // -- [ Mesh generation ] -- //
    let vertices = triangles
        .iter()
        .map(|triangle| [triangle.a, triangle.b, triangle.c])
        .flatten()
        .map(|vector| [vector.x, vector.y, vector.z])
        .collect::<Vec<_>>();

    let indices = (0..vertices.len())
        .map(|index| index as u32)
        .collect::<Vec<u32>>();

    let uvs = (0..vertices.len())
        .map(|_| [0.0, 0.0])
        .collect::<Vec<[f32; 2]>>();

    let mut normals: Vec<[f32; 3]> = Vec::new();

    for triangle in indices.chunks(3) {
        let a = Vec3::from(vertices[(triangle)[0] as usize]);
        let b = Vec3::from(vertices[(triangle)[1] as usize]);
        let c = Vec3::from(vertices[(triangle)[2] as usize]);

        let normal = (b - a).cross(c - a).normalize();

        normals.push(normal.into());
        normals.push(normal.into());
        normals.push(normal.into());
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.set_indices(Some(Indices::U32(indices)));

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.8).into()),
        ..default()
    });
}


fn get_cube_index(chunk: &mut Chunk, points: [u8; 8]) -> u16 {
    let mut cube_index = 0;

    for i in 0..points.len() {
        if chunk.surface_level < points[i] {
            cube_index |= 1 << i;
        }
    }

    cube_index
}

fn index_to_vec3(size: u32, index: u32) -> Vec3 {

    let x = index % size;
    let y = index / u32::pow(size, 2);
    let z = (index / size) % size;

    Vec3::new(x as f32, y as f32, z as f32)
}

fn get_cube_points(chunk: &mut Chunk, index: u32) -> [u8; 8] {
    let mut points = [0; 8];
    let size = chunk.size;

    let x = index % size;
    let y = index / u32::pow(size, 2);
    let z = (index / size) % size;

    // 8x8x8 Point cloud
    // 7x7x7 cube grid
    // EG: index 2(0..) => vertex = [3, 4, 11, 12, 67, 68, 75, 76]

    let point_index = |x, y, z| {
        (x + y * (size + 1) + z * u32::pow(size + 1, 2)) as usize
    };

    // -- Top --
    points[0] = chunk.vertex_vector[point_index(x, y, z)];
    points[1] = chunk.vertex_vector[point_index(x + 1, y, z)];
    points[2] = chunk.vertex_vector[point_index(x, y + 1, z)];
    points[3] = chunk.vertex_vector[point_index(x + 1, y + 1, z)];

    // -- Bottom --
    points[4] = chunk.vertex_vector[point_index(x, y, z + 1)];
    points[5] = chunk.vertex_vector[point_index(x + 1, y, z + 1)];
    points[6] = chunk.vertex_vector[point_index(x, y + 1, z + 1)];
    points[7] = chunk.vertex_vector[point_index(x + 1, y + 1, z + 1)];

    points
}


fn corner_coordinates(chunk: &mut Chunk, index: u32) -> [Vec3; 8] {
    let size = chunk.size;

    let x = index % size;
    let y = index / u32::pow(size, 2);
    let z = (index / size) % size;

    let mut coordinates: [Vec3; 8] = [Vec3::new(0.0, 0.0, 0.0); 8];

    coordinates[0] = Vec3::new(x as f32, y as f32, z as f32);
    coordinates[1] = Vec3::new(x as f32 + 1.0, y as f32, z as f32);
    coordinates[2] = Vec3::new(x as f32, y as f32 + 1.0, z as f32);
    coordinates[3] = Vec3::new(x as f32 + 1.0, y as f32 + 1.0, z as f32);
    coordinates[4] = Vec3::new(x as f32, y as f32, z as f32 + 1.0);
    coordinates[5] = Vec3::new(x as f32 + 1.0, y as f32, z as f32 + 1.0);
    coordinates[6] = Vec3::new(x as f32, y as f32 + 1.0, z as f32 + 1.0);
    coordinates[7] = Vec3::new(x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0);

    coordinates 
}
 