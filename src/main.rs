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

use piston_window::{Key, ControllerButton, ControllerHat, HatState};

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

    let mut keyboard_map = InputRoleMap::<Key>::new();
    {
        let set_list = [
            (Key::D1,    InputRole::Button0),
            (Key::D2,    InputRole::Button1),
            (Key::D3,    InputRole::Button2),
            (Key::D4,    InputRole::Button3),
            (Key::Z,     InputRole::Button4),
            (Key::X,     InputRole::Button5),
            (Key::C,     InputRole::Button6),
            (Key::Space, InputRole::Button7),
            (Key::Space, InputRole::Button4),
            (Key::W,     InputRole::Up),
            (Key::D,     InputRole::Right),
            (Key::S,     InputRole::Down),
            (Key::A,     InputRole::Left),
            (Key::E,     InputRole::Up),
            (Key::E,     InputRole::Right),
            (Key::Up,    InputRole::Up),
            (Key::Right, InputRole::Right),
            (Key::Down,  InputRole::Down),
            (Key::Left,  InputRole::Left),
        ];
        keyboard_map.assign(&set_list);
    }
    let mut button_map = InputRoleMap::<ControllerButton>::new();
    {
        let set_list = [
            (ControllerButton {id: 0, button: 0}, InputRole::Left),
            (ControllerButton {id: 0, button: 1}, InputRole::Down),
            (ControllerButton {id: 0, button: 2}, InputRole::Right),
            (ControllerButton {id: 0, button: 3}, InputRole::Up),
        ];
        button_map.assign(&set_list);
    }
    let mut hat_map = InputRoleMap::<ControllerHat>::new();
    {
        let set_list = [
            (ControllerHat {id: 0, which: 0, state: HatState::Up}, InputRole::Up),
            (ControllerHat {id: 0, which: 0, state: HatState::Right}, InputRole::Right),
            (ControllerHat {id: 0, which: 0, state: HatState::Down}, InputRole::Down),
            (ControllerHat {id: 0, which: 0, state: HatState::Left}, InputRole::Left),
            (ControllerHat {id: 0, which: 0, state: HatState::RightUp}, InputRole::Right),
            (ControllerHat {id: 0, which: 0, state: HatState::RightUp}, InputRole::Up),
            (ControllerHat {id: 0, which: 0, state: HatState::RightDown}, InputRole::Right),
            (ControllerHat {id: 0, which: 0, state: HatState::RightDown}, InputRole::Down),
            (ControllerHat {id: 0, which: 0, state: HatState::LeftUp}, InputRole::Left),
            (ControllerHat {id: 0, which: 0, state: HatState::LeftUp}, InputRole::Up),
            (ControllerHat {id: 0, which: 0, state: HatState::LeftDown}, InputRole::Left),
            (ControllerHat {id: 0, which: 0, state: HatState::LeftDown}, InputRole::Down),
        ];
        hat_map.assign(&set_list);
    }
    let mut input_role_state = InputRoleState::new();

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

    let mut controller_axis_args: Vec<piston_window::ControllerAxisArgs> = Vec::new();
    let mut s0_pos = 0.0;
    let mut s1_pos = 0.0;
    let mut s2_pos = 0.0;
    let mut s3_pos = 0.0;

    input_role_state.clear_all();
    'main_loop: loop {
        bg.0.set_cur_pos(0, 0).put_string(&format!("{}", t_count), None);
        if input_role_state.get(InputRole::Up).0 {
            obj_pos.y -= 1;
        }
        if input_role_state.get(InputRole::Down).0 {
            obj_pos.y += 1;
        }
        if input_role_state.get(InputRole::Left).0 {
            obj_pos.x -= 1;
        }
        if input_role_state.get(InputRole::Right).0 {
            obj_pos.x += 1;
        }
        bg.0.set_cur_pos(0, 1).put_string(&format!("({:3},{:3})", obj_pos.x, obj_pos.y), Some(&CharAttributes::new(3, BgSymmetry::Normal)));
        spr.sp(0).pos(obj_pos);
        bg.1.set_view_pos(obj_pos.x, obj_pos.y);

        bg.0.set_cur_pos(0, 2).put_string(&format!("{}  ", controller_axis_args.len()), None);
        for args in controller_axis_args.iter() {
            match args.axis {
                0 => s0_pos = args.position,
                1 => s1_pos = args.position,
                2 => s2_pos = args.position,
                3 => s3_pos = args.position,
                _ => {}
            }
        }
        bg.0.set_cur_pos(0, 3).put_string(&format!("{}                                        ", s0_pos), None);
        bg.0.set_cur_pos(0, 4).put_string(&format!("{}                                        ", s1_pos), None);
        bg.0.set_cur_pos(0, 5).put_string(&format!("{}                                        ", s2_pos), None);
        bg.0.set_cur_pos(0, 6).put_string(&format!("{}                                        ", s3_pos), None);

        if wait_and_update::doing(&mut game_window, &mut spr, &mut bg, &mut keyboard_map, &mut button_map, &mut hat_map, &mut controller_axis_args) {
            break 'main_loop;
        }
        input_role_state.clear_state();
        input_role_state.update_state(&keyboard_map);
        input_role_state.update_state(&button_map);
        input_role_state.update_state(&hat_map);
        input_role_state.update_history();
        t_count += 1;
    }
    sdl_context.mouse().show_cursor(true);
}
