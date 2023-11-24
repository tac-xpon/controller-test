mod bgsp_data;
use bgsp_data::*;

mod direction;
use direction::*;

mod input_role;
use input_role::*;

mod game_window;
use game_window::*;

mod wait_and_update;

use bgsp_lib2::{
    bgsp_common::*,
    bg_plane::*,
    sp_resources::*,
};

use std::collections::BTreeMap;

const FULL_SCREEN: bool = false;
const VM_RECT_SIZE: (i32, i32) = (32, 32);
const VM_RECT_PIXEL_SIZE: (i32, i32) = (VM_RECT_SIZE.0 * PATTERN_SIZE as i32, VM_RECT_SIZE.1 * PATTERN_SIZE as i32);
const ROTATION: Direction = Direction::Up;
const PIXEL_SCALE: i32 = 2;
const WINDOW_MARGIN: i32 = 2;
const BG0_RECT_SIZE: (i32, i32) = (64, 64);
const BG1_RECT_SIZE: (i32, i32) = (64, 64);
const MAX_SPRITES: usize = 128;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // let audio_subsystem = sdl_context.audio().unwrap();
    let mut game_window = GameWindow::new(
        video_subsystem,
        FULL_SCREEN,
        VM_RECT_PIXEL_SIZE,
        ROTATION,
        PIXEL_SCALE,
        WINDOW_MARGIN,
    );

    let mut keyboard_map: BTreeMap<piston_window::Key, Vec<_>> = BTreeMap::new();
    {
        let key_set_list = [
            (piston_window::Key::D1,    InputRole::Button0),
            (piston_window::Key::D2,    InputRole::Button1),
            (piston_window::Key::D3,    InputRole::Button2),
            (piston_window::Key::D4,    InputRole::Button3),
            (piston_window::Key::Z,     InputRole::Button4),
            (piston_window::Key::X,     InputRole::Button5),
            (piston_window::Key::C,     InputRole::Button6),
            (piston_window::Key::Space, InputRole::Button7),
            (piston_window::Key::Space, InputRole::Button4),
            (piston_window::Key::W,     InputRole::Up),
            (piston_window::Key::D,     InputRole::Right),
            (piston_window::Key::S,     InputRole::Down),
            (piston_window::Key::A,     InputRole::Left),
            (piston_window::Key::E,     InputRole::Up),
            (piston_window::Key::E,     InputRole::Right),
            (piston_window::Key::Up,    InputRole::Up2),
            (piston_window::Key::Right, InputRole::Right2),
            (piston_window::Key::Down,  InputRole::Down2),
            (piston_window::Key::Left,  InputRole::Left2),
        ];
        for key_set in key_set_list {
            if let Some(role_list) = keyboard_map.get_mut(&key_set.0) {
                role_list.push(key_set.1);
            } else {
                keyboard_map.insert(key_set.0, vec![key_set.1]);
            }
        }
    }
    let mut input_role_state = InputRoleState::default();

    let mut bg_texture_bank = BgTextureBank::new(
        &bgchar_data::BG_PATTERN_TBL,
        &bgpal_data::COLOR_TBL,
        game_window.pixel_scale() as i32,
    );
    let rc_bg_texture_bank = Rc::new(RefCell::new(&mut bg_texture_bank));
    let mut bg = {
        let bg0 = BgPlane::new(
            BG0_RECT_SIZE,
            VM_RECT_PIXEL_SIZE,
            rc_bg_texture_bank.clone(),
        );

        let bg1 = BgPlane::new(
            BG1_RECT_SIZE,
            VM_RECT_PIXEL_SIZE,
            rc_bg_texture_bank.clone(),
        );
        (bg0, bg1)
    };

    let mut sp_texture_bank = SpTextureBank::new(
        &spchar_data::SP_PATTERN_TBL,
        &sppal_data::COLOR_TBL,
        game_window.pixel_scale() as i32,
    );
    let rc_sp_texture_bank = Rc::new(RefCell::new(&mut sp_texture_bank));
    let mut spr = SpResources::new(
        MAX_SPRITES,
        rc_sp_texture_bank.clone(),
    );

    let mut t_count = 0;
    let mut obj_pos = SpPos::new(0, 0);
    bg.0.fill_palette(1);
    bg.0.set_cur_pos(10, 10).put_string("Hello, world!", Some(&CharAttributes::new(4, BgSymmetry::Normal)));
    {
        let mut c = 0u32;
        for y in 0..BG1_RECT_SIZE.1 {
            for x in 0..BG1_RECT_SIZE.0 {
                bg.1.set_code_at(x, y, c + 0x20).set_palette_at(x, y, 5);
                c = (c + 1) % (0x80 - 0x20);
            }
        }
    }
    spr.sp(0).pos(obj_pos).code(0).palette(1).visible(true);

    input_role_state.clear_all();
    'main_loop: loop {
        bg.0.set_cur_pos(0, 0).put_string(&format!("{}", t_count), None);
        if input_role_state.get(InputRole::Up2).0 {
            obj_pos.y -= 1;
        }
        if input_role_state.get(InputRole::Down2).0 {
            obj_pos.y += 1;
        }
        if input_role_state.get(InputRole::Left2).0 {
            obj_pos.x -= 1;
        }
        if input_role_state.get(InputRole::Right2).0 {
            obj_pos.x += 1;
        }
        bg.0.set_cur_pos(0, 1).put_string(&format!("({:3},{:3})", obj_pos.x, obj_pos.y), Some(&CharAttributes::new(3, BgSymmetry::Normal)));
        spr.sp(0).pos(obj_pos);
        bg.1.set_view_pos(obj_pos.x, obj_pos.y);
        if wait_and_update::doing(&mut game_window, &mut spr, &mut bg, &keyboard_map, &mut input_role_state) {
            break 'main_loop;
        }
        t_count += 1;
    }
    sdl_context.mouse().show_cursor(true);
}
