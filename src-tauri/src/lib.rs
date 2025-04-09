// src-tauri/src/lib.rs

use rand::Rng;
use serde::Serialize;
use std::{borrow::BorrowMut, sync::Mutex};
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
    is_center: bool
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
            is_center: false
        }
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
        let g = 6.67430e-11; // 引力常数
        let mut bodies_clone = self.bodies.clone();

        for i in 0..self.bodies.len() {
            let body = &mut self.bodies[i];
            for j in 0..bodies_clone.len() {
                if i == j {
                    continue;
                }
                let other = &mut bodies_clone[j].borrow_mut();
                let dx = other.x - body.x;
                let dy = other.y - body.y;
                let distance_sq = dx * dx + dy * dy;
                let distance = distance_sq.sqrt();

                if distance < body.radius + other.radius + dd {
                    // 简单的碰撞处理：停止移动
                    if body.radius >= other.radius {
                        body.is_center = true;
                        body.orbit_radius = body.radius + other.radius + dd;
                        other.angle += 0.05;
                        other.x = body.x + body.orbit_radius * other.angle.cos();
                        other.y = body.y + body.orbit_radius * other.angle.sin();
                    } else {
                        other.is_center = true;
                        body.orbit_radius = body.radius + other.radius + dd;
                        body.angle += 0.05;
                        body.x = other.x + body.orbit_radius * body.angle.cos();
                        body.y = other.y + body.orbit_radius * body.angle.sin();
                    }
                    
                    // return;
                } else {
                    // 计算引力
                    body.is_center = false;
                    let force = g * body.mass * other.mass / distance_sq;
                    body.vx += force * dx / distance / body.mass;
                    body.vy += force * dy / distance / body.mass;
                }
            }

            // 更新位置
            body.x += body.vx;
            body.y += body.vy;

            // 边界反弹
            if body.x < body.radius {
                body.x = body.radius;
                body.vx *= -1.0;
            } else if body.x > self.canvas_width - body.radius {
                body.x = self.canvas_width - body.radius;
                body.vx *= -1.0;
            }

            if body.y < body.radius {
                body.y = body.radius;
                body.vy *= -1.0;
            } else if body.y > self.canvas_height - body.radius {
                body.y = self.canvas_height - body.radius;
                body.vy *= -1.0;
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
        let mass = rng.gen_range(1.0..100.0);
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

