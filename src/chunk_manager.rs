use std::collections::HashMap;

use glow::*;
use glam::Mat4;
use crate::chunk::*;
use crate::kmath::*;
use crate::priority_queue::*;
// use crate::world_gen::*;
use crate::world_gen2::*;
use crate::settings::*;
use crate::camera::*;
use crossbeam::*;
use crossbeam_channel::*;
use std::collections::HashSet;

/*
Responsibilities:
1. Hold chunks
2. Decide to allocate chunks
3. Decide to deallocate chunks
4. Sort them for rendering?

*/

// idea to handle floating point precision: just mod everything and tell the chunk where it is when we ask it to mesh / draw, but theres an edge case as there always is :)

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChunkCoordinates {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkCoordinates {
    pub fn containing_world_pos(pos: Vec3) -> ChunkCoordinates {
        let ccf = pos / S as f32;
        let x = ccf.x.floor() as i32;
        let y = ccf.y.floor() as i32;
        let z = ccf.z.floor() as i32;
        ChunkCoordinates {x, y, z}
    }

    pub fn center(&self) -> Vec3 {
        Vec3::new(
            self.x as f32 * S_F32 + HALF_S_F32,
            self.y as f32 * S_F32 + HALF_S_F32,
            self.z as f32 * S_F32 + HALF_S_F32,
        )
    }

    pub fn corners(&self) -> [Vec3; 8] {
        let x = self.x as f32 * S_F32;
        let y = self.y as f32 * S_F32;
        let z = self.z as f32 * S_F32;
        
        [
            Vec3::new(x,y,z),
            Vec3::new(x,y,z+S_F32),
            Vec3::new(x,y+S_F32,z),
            Vec3::new(x,y+S_F32,z+S_F32),
            Vec3::new(x+S_F32,y,z),
            Vec3::new(x+S_F32,y,z+S_F32),
            Vec3::new(x+S_F32,y+S_F32,z),
            Vec3::new(x+S_F32,y+S_F32,z+S_F32),
        ]
    }
}

pub struct ChunkManager {
    pub chunk_map: HashMap<ChunkCoordinates, Chunk>,
    //chunks_to_generate: PriorityQueue<f32, ChunkCoordinates>,

    job_sender: Sender<ChunkCoordinates>,
    chunk_receiver: Receiver<(ChunkData, (Vec<f32>, Vec<u32>), (Vec<f32>, Vec<u32>))>,    // might be doing unnecessary copying
    loading: HashSet<ChunkCoordinates>,
    gen: WorldGen,
}

impl ChunkManager {
    pub fn new(gl: &glow::Context, gen: WorldGen) -> ChunkManager {
        let mut chunk_map = HashMap::new();

        let (job_sender, job_receiver) = unbounded();
        let (chunk_sender, chunk_receiver) = unbounded();

        for i in 0..N_WORKERS {
            let job_receiver =  job_receiver.clone();
            let chunk_sender = chunk_sender.clone();
            // let gen = gen.clone();
            std::thread::spawn(move || {
                let thread_gen = gen.clone();

                loop {
                    let job = job_receiver.recv().unwrap();
                    let chunk_data = ChunkData::new(job, &thread_gen);
                    let opaque_stuff = chunk_data.opaque_buffers_opt();
                    let transparent_stuff = chunk_data.transparent_buffers_opt();
                    chunk_sender.send((chunk_data, opaque_stuff, transparent_stuff)).unwrap();
                }
            });
        }

        ChunkManager {
            chunk_map,
            job_sender,
            chunk_receiver,
            loading: HashSet::new(),
            gen,
        }
    }

    pub fn draw(&self, gl: &glow::Context, cam: &Camera) {

        // println!("pos: {}\nlook: {}\n up: {}\n right: {}\n fovx: {}\n fovy: {}", pos, look, up, right, fovx, fovy);


        let mut draw_list: Vec<&Chunk> = self.chunk_map.iter().filter(|(cc, c)| {
            let corners = cc.corners();
            for corner in corners {
                if cam.point_in_vision(corner) {
                    return true;
                }
            }
            return false;
        }).map(|x| x.1).collect();

        // println!("draw {} / {}", draw_list.len(), self.chunk_map.len());
        
        draw_list.sort_unstable_by(|chunk1, chunk2| {
            let dist1 = (chunk1.data.cc.center() - cam.pos).square_distance();
            let dist2 = (chunk2.data.cc.center() - cam.pos).square_distance();
            dist1.partial_cmp(&dist2).unwrap()
        });

        for chunk in draw_list.iter().rev() {
            if let Some(m) = &chunk.opaque_mesh {
                m.draw(gl);
            }
        }

        for chunk in draw_list.iter() {
            if let Some(m) = &chunk.transparent_mesh {
                m.draw(gl);
            }
        }
    }

    pub fn treadmill(&mut self, gl: &glow::Context, cam: &Camera) {
        let in_chunk = ChunkCoordinates::containing_world_pos(cam.pos);

        self.chunk_map.retain(|cc, chunk| {
            let x = cc.x;
            let y = cc.y;
            let z = cc.z;

            let keep =(x - in_chunk.x).abs() <= CHUNK_RADIUS &&
            (y - in_chunk.y).abs() <= CHUNK_RADIUS/3 &&
            (z - in_chunk.z).abs() <= CHUNK_RADIUS;

            if !keep {
                chunk.destroy(gl);
            }

            keep
        });

        let mut new_jobs = Vec::new();

        // post jobs
        for i in -CHUNK_RADIUS..=CHUNK_RADIUS {
            for j in -CHUNK_RADIUS/3..=CHUNK_RADIUS/3 {
                for k in -CHUNK_RADIUS..=CHUNK_RADIUS {
                    let x = in_chunk.x + i;
                    let y = in_chunk.y + j;
                    let z = in_chunk.z + k;

                    let cc = ChunkCoordinates {x,y,z};

                    if !self.chunk_map.contains_key(&cc) && !self.loading.contains(&cc) {
                        new_jobs.push(cc);
                    }
                }
            }
        }

        new_jobs.retain(|cc| !self.loading.contains(cc));
        new_jobs.sort_by_key(|cc| {
            let distance = (cam.pos - cc.center()).magnitude();
            let in_view = cam.point_in_vision(cc.center()); // probably rough around the edges

            // could do surface heuristic too

            -(distance * match in_view {
                true => 0.1,
                false => 1.0,
            } * 1000.0) as i32
        });

        let watermark = 60;
        while self.loading.len() < watermark {
            if let Some(job) = new_jobs.pop() {
                self.loading.insert(job);
                self.job_sender.send(job).unwrap();
            } else {
                break;
            }
        }

        let mut chunks_this_frame = 0;
        // reap chunks
        while let Ok((chunk_data, (ov, oe), (tv, te))) = self.chunk_receiver.try_recv() {
            let opaque_mesh = new_opaque_mesh(gl, &ov, &oe);
            let transparent_mesh = new_transparent_mesh(gl, &tv, &te);

            let new_chunk = Chunk {
                data: chunk_data,
                opaque_mesh,
                transparent_mesh,
            };
            self.loading.remove(&new_chunk.data.cc);
            self.chunk_map.insert(new_chunk.data.cc, new_chunk);
            chunks_this_frame += 1;
            if chunks_this_frame > CHUNKS_PER_FRAME {
                break;
            }
        }
    }
}