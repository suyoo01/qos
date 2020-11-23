use crate::mem::{self, memcpy};
use crate::paging::PAGE_SIZE;
use crate::{paging, util};
use crate::{println, print};

#[repr(C)]
pub struct Env {
    pub tf: TrapFrame,
    pub link: usize,
    pub id: usize,
    pub parent_id: usize,
    pub env_type: EnvType,
    pub status: usize,
    pub runs: usize,
    pub pgdir: usize
}

#[derive(Debug)]
#[repr(C)]
pub struct TrapFrame {
    pub reg: [usize;16]
}

pub enum EnvType{}


pub struct UserEnv {
    envs: [Env; 1024]
}

pub static mut ENVS:usize = 0;

pub fn env_init() {
}

fn get_envs() -> &'static mut UserEnv{
    unsafe {
        &mut *(ENVS as *mut UserEnv)
    }
}


/// Create first user env
pub fn env_create(binary: usize) {
    let new_env = env_alloc(0);
    println!("{:x}", binary);
    
    unsafe {
        load_icode(new_env, binary);
        env_run(0);
    }
}

pub unsafe fn env_run(id: usize) {
    let envs = &*(ENVS as *const UserEnv);
    paging::change_pgdir(envs.envs[id].pgdir);
    env_pop_tf(&envs.envs[id].tf);
}

fn get_free_env() -> usize {
    0
}

pub fn env_alloc(parent_id: usize) -> &'static mut Env {
    let envs = get_envs();

    let new_env = &mut envs.envs[get_free_env()];
    new_env.parent_id = parent_id;


    env_setup_vm(new_env);
    new_env
}

pub fn env_setup_vm(env: &mut Env) {
    let current = mem::fn_to_pa(mem::alloc_frame(0, 0));
    let padding = (util::round_up(current, PAGE_SIZE * 4)-current)/PAGE_SIZE;

    env.pgdir = mem::fn_to_pa(mem::alloc_frame(padding + 4, 0));
    env.pgdir = util::round_up(env.pgdir, PAGE_SIZE * 4);
    unsafe {
        let pgdir = mem::pa_to_va(env.pgdir) as *mut u8;
        let offset = (paging::UTOP/paging::SECTION_SIZE) as isize;
        memcpy(pgdir,
            paging::get_page_table() as *const _ as *const u8, 
            PAGE_SIZE * 4);
    }
}

unsafe fn load_icode(env: &mut Env, binary: usize) {
    use crate::elf::{ElfHeader, ProgramHeader};

    let old_pgdir = paging::change_pgdir(env.pgdir);
    let binary = binary as *mut u8;
    let elf_hdr = &*(binary as *const ElfHeader);
    println!("{:x}", elf_hdr.e_ident[0]);
    if elf_hdr.e_ident[0] != 0x464c457f{
        panic!("Not a valid elf");
    }
    let prog_hdr = binary.offset(elf_hdr.e_phoff as isize) as *const ProgramHeader;
    for ph_num in 0..elf_hdr.e_phnum {
        let ph = &*prog_hdr.offset(ph_num as isize);
        if ph.p_type != 1 {
            continue;
        }
        println!("Load to {:x} ({:x} bytes)", ph.p_paddr, ph.p_filesz);
        let start = util::round_down(ph.p_paddr as usize, PAGE_SIZE);
        let end = util::round_up((ph.p_paddr + ph.p_memsz) as usize, PAGE_SIZE);
        let num_frames = (end-start) / PAGE_SIZE;
        let frame_number = mem::alloc_frame(num_frames, 1);

        for i in 0..num_frames {
            paging::map_va_to_fn(start+PAGE_SIZE*i, frame_number+i, paging::USER_FLAG);
        }
        memcpy(ph.p_paddr as *mut u8, binary.offset(ph.p_offset as isize), ph.p_filesz as usize);
    }
    let user_stack = mem::alloc_frame(1, 0);
    paging::map_va_to_fn(paging::USTACK - PAGE_SIZE, user_stack, paging::USER_FLAG);

    env.tf.reg[15] = elf_hdr.e_entry as usize;
    env.tf.reg[13] = paging::USTACK;

    paging::change_pgdir(old_pgdir);
}

unsafe fn env_pop_tf(tf :&TrapFrame) {
    asm!(
    "
    msr cpsr_c, #0x10
    add r1, {val0}, #0
    add r2, {val1}, #0
    mov sp, r1
    mov pc, r2
    ",
    val0 = in(reg) tf.reg[13],
    val1 = in(reg) tf.reg[15],)
}