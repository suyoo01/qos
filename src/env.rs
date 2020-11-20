use spin::Mutex;

pub static ENVS: Mutex<usize> = Mutex::new(0);

pub struct Env {
    tf: TrapFrame,
    link: usize,
    id: usize,
    parent_id: usize,
    env_type: EnvType,
    status: usize,
    runs: usize,
    pgdir: usize
}

enum EnvType{}

#[repr(C)]
pub struct TrapFrame {
    
}

pub struct UserEnv {
    envs: [Env; 1024]
}

