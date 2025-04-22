// src-tauri/src/lib.rs

use rand::Rng;
use serde::Serialize;
use std::sync::Mutex;
use tauri::{command, State};

// 定义 NBody 结构体，作为纯数据结构进行序列化
#[derive(Clone, Serialize)]
struct NBody {
    id: i32,
    x: f64,
    y: f64,
    mass: f64,
    radius: f64,
    vx: f64,
    vy: f64,
    angle: f64,
    orbit_radius: f64,
    is_center: bool,
}

impl NBody {
    fn new(id: i32, x: f64, y: f64, mass: f64, radius: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            x,
            y,
            mass,
            radius,
            vx: rng.gen_range(-1.0..1.0),
            vy: rng.gen_range(-1.0..1.0),
            angle: 0.0,
            orbit_radius: 0.0,
            is_center: false,
        }
    }

    fn in_aabb(&self, other: &NBody, dd: f64) -> bool {
        let dist_sp = (other.x - self.x).powi(2) + (other.y - self.y).powi(2);
        if dist_sp < dd + other.radius + self.radius && dist_sp > 0.0 {
            true
        } else {
            false
        }
    }

    fn set_is_center(&mut self, is_center: bool) {
        self.is_center = is_center;
    }
}

// 定义 Simulation 结构体，用于管理天体和模拟状态
struct Simulation {
    bodies: Vec<NBody>,
    canvas_width: f64,
    canvas_height: f64,
}

impl Simulation {
    fn new() -> Self {
        Self {
            bodies: Vec::new(),
            canvas_width: 800.0,  // 默认值
            canvas_height: 600.0, // 默认值
        }
    }

    fn update_canvas_size(&mut self, width: f64, height: f64) {
        self.canvas_width = width;
        self.canvas_height = height;
    }

    fn update_simulation(&mut self, dd: f64) {
        let g = 6.67430e-11;
        let len = self.bodies.len();

        // 1) 两层循环，i < j，保证每对处理一次
        for i in 0..len {
            'other: for j in (i + 1)..len {
                // split_at_mut(j) 把 Vec 分成 [0..j] 和 [j..]
                let (left, right) = self.bodies.split_at_mut(j);
                let body_i = &mut left[i];
                let body_j = &mut right[0];
                if body_i.in_aabb(body_j, dd) {
                    continue 'other;
                }

                // 2) 计算 i 与 j 间的距离和引力/碰撞
                let dx = body_j.x - body_i.x;
                let dy = body_j.y - body_i.y;
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt();

                if dist < body_i.radius + body_j.radius + dd {
                    // 碰撞逻辑（示例）
                    if body_i.radius >= body_j.radius {
                        body_i.set_is_center(true);
                        body_i.orbit_radius = body_i.radius + body_j.radius + dd;
                        body_j.angle += 0.05;
                        body_j.x = body_i.x + body_i.orbit_radius * body_j.angle.cos();
                        body_j.y = body_i.y + body_i.orbit_radius * body_j.angle.sin();
                    } else {
                        body_j.set_is_center(true);
                        body_j.orbit_radius = body_i.radius + body_j.radius + dd;
                        body_i.angle += 0.05;
                        body_i.x = body_j.x + body_j.orbit_radius * body_i.angle.cos();
                        body_i.y = body_j.y + body_j.orbit_radius * body_i.angle.sin();
                    }
                } else {
                    body_i.set_is_center(false);
                    body_j.set_is_center(false);
                    // 万有引力计算，注意作用力是等大反向
                    let force = g * body_i.mass * body_j.mass / dist_sq;
                    let ax = force * dx / dist / body_i.mass;
                    let ay = force * dy / dist / body_i.mass;
                    body_i.vx += ax;
                    body_i.vy += ay;
                    body_j.vx -= ax;
                    body_j.vy -= ay;
                }
            }
        }

        // 3) 最后统一更新所有天体的位置和边界反弹
        for body in &mut self.bodies {
            body.x += body.vx;
            body.y += body.vy;
            // 边界反弹
            if body.x < body.radius {
                body.x = body.radius;
                body.vx = -body.vx;
            } else if body.x > self.canvas_width - body.radius {
                body.x = self.canvas_width - body.radius;
                body.vx = -body.vx;
            }
            if body.y < body.radius {
                body.y = body.radius;
                body.vy = -body.vy;
            } else if body.y > self.canvas_height - body.radius {
                body.y = self.canvas_height - body.radius;
                body.vy = -body.vy;
            }
        }
    }
}

// 使用 Mutex 进行线程安全管理
type SimulationState = Mutex<Simulation>;

// Tauri 命令：问候函数
#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Tauri 命令：创建单个天体
#[command]
fn create_body(
    id: i32,
    x: f64,
    y: f64,
    mass: f64,
    radius: f64,
    state: State<'_, SimulationState>,
) -> NBody {
    let body = NBody::new(id, x, y, mass, radius);
    let mut sim = state.lock().unwrap();
    sim.bodies.push(body.clone());
    body
}

// Tauri 命令：创建多个天体
#[command]
fn create_bodies(
    count: usize,
    canvas_width: f64,
    canvas_height: f64,
    state: State<'_, SimulationState>,
) -> Vec<NBody> {
    let mut sim = state.lock().unwrap();
    sim.bodies.clear();
    sim.update_canvas_size(canvas_width, canvas_height);
    let mut rng = rand::thread_rng();

    for i in 0..count {
        let radius = rng.gen_range(5.0..15.0);
        let mass = radius * 10.0 * 5.0;
        let x = rng.gen_range(radius..(canvas_width - radius));
        let y = rng.gen_range(radius..(canvas_height - radius));
        let body = NBody::new(i as i32, x, y, mass, radius);
        sim.bodies.push(body);
    }

    sim.bodies.clone()
}

// Tauri 命令：更新模拟
#[command]
fn update_simulation(dd: f64, state: State<'_, SimulationState>) -> Vec<NBody> {
    let mut sim = state.lock().unwrap();
    sim.update_simulation(dd);
    sim.bodies.clone()
}

// Tauri 命令：获取当前所有天体
#[command]
fn get_bodies(state: State<'_, SimulationState>) -> Vec<NBody> {
    let sim = state.lock().unwrap();
    sim.bodies.clone()
}

// Tauri 命令：更新画布大小
#[command]
fn update_canvas_size(
    width: f64,
    height: f64,
    state: State<'_, SimulationState>,
) -> Result<(), String> {
    let mut sim = state.lock().map_err(|e| e.to_string())?;
    sim.update_canvas_size(width, height);
    Ok(())
}

// Tauri 应用程序入口
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 将 Simulation 包装在 Mutex 中进行管理
        .manage(Mutex::new(Simulation::new()))
        .invoke_handler(tauri::generate_handler![
            greet,
            create_body,
            create_bodies,
            update_simulation,
            get_bodies,
            update_canvas_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
