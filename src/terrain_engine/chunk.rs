use bevy::{prelude::*, render::mesh::{PrimitiveTopology, Indices}};

use crate::components::{Chunk, Triangle, Point};
mod marching_cube_table;
use marching_cube_table::*;

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

        let pos = Vec3::new(
            (x % (chunk.size + 1)) as f32,
            (x / (chunk.size + 1) % (chunk.size + 1)) as f32,
            (x / (chunk.size + 1).pow(2)) as f32,
        );  

        chunk.point_cloud.push(Point {
            level: random_number,
            pos,
        });
    }


    // -- [ Marching cube ] -- //
    let mut triangles = Vec::new();

    for cube in 0..u32::pow(chunk.size, 3) {
        let cloud_points = get_cube_points(index_to_vec3(8, cube), &mut chunk); 

        let mut points:[u8; 8] = [0; 8];
        let mut coords:[Vec3; 8] = [Vec3::default(); 8];

        for (i, p) in cloud_points.iter().enumerate() {
            points[i] = p.level;
            coords[i] = p.pos;
        }

        let cube_index = get_cube_index(&mut chunk, points);

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

            triangle.a = (coords[a0 as usize] + coords[a1 as usize]) * 0.5;
            triangle.b = (coords[b0 as usize] + coords[b1 as usize]) * 0.5;
            triangle.c = (coords[c0 as usize] + coords[c1 as usize]) * 0.5;

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

fn get_cube_points(cube_pos: Vec3, chunk: &mut Chunk) -> Vec<Point> {
    // --[ Get the 8 points of the cube ]-- //
    let mut points_pos = Vec::new();
    
    points_pos.push(cube_pos + Vec3::new(0.0, 0.0, 0.0));
    points_pos.push(cube_pos + Vec3::new(1.0, 0.0, 0.0));
    points_pos.push(cube_pos + Vec3::new(1.0, 1.0, 0.0));
    points_pos.push(cube_pos + Vec3::new(0.0, 1.0, 0.0));
    points_pos.push(cube_pos + Vec3::new(0.0, 0.0, 1.0));
    points_pos.push(cube_pos + Vec3::new(1.0, 0.0, 1.0));
    points_pos.push(cube_pos + Vec3::new(1.0, 1.0, 1.0));
    points_pos.push(cube_pos + Vec3::new(0.0, 1.0, 1.0));


    // --[ Locate the 8 points ]-- //
    let mut points = Vec::new();

    for i in 0..points_pos.len() {
        points.push(chunk.get_point(points_pos[i]));
    }

    points
}
 