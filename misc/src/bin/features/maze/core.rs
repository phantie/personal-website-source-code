#![allow(unused, deprecated)]

use misc::features::maze::singleplayer_movement_state::*;
use misc::features::maze::*;

fn main() {
    'a: {
        break 'a;

        let m = test_mazes::n0();
        let m = pad_matrix(m);
        test_mazes::simple_display_matrix(&m);
    }

    'b: {
        break 'b;

        for d in vec![
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ] {
            dbg!(d);

            let m = test_mazes::n0();
            let pos = test_mazes::n0_start();
            let mut m = pad_matrix(m);
            let pos = pad_position(pos);

            let mut s = VisitState::new_from_padded(m.clone());

            s.validate_pos(pos);

            let steps_to_direction = movement_possibility(&m, d, pos);
            dbg!(steps_to_direction);

            let m = pad_matrix(m);
            let m = align_matrix(m, d);
            test_mazes::simple_display_matrix(&m);
        }
    }

    'c: {
        break 'c;

        let m = test_mazes::n0();
        let pos = test_mazes::n0_start();
        let mut m = pad_matrix(m);
        let pos = pad_position(pos);

        let mut s = VisitState::new_from_padded(m.clone());

        s.validate_pos(pos);
        test_mazes::simple_display_matrix(&m);
        println!();
        test_mazes::simple_display_discovered_matrix(&m, pos);
        println!();

        let mut can_move_to = can_move_to_directions(&m, pos);
        println!("You can go to {can_move_to:?}");

        let pos = move_to_direction_once(&mut m, Direction::Right, pos);
        test_mazes::simple_display_discovered_matrix(&s.m, pos);
        println!();
        let mut can_move_to = can_move_to_directions(&m, pos);
        println!("You can go to {can_move_to:?}");

        let pos = move_to_direction_once(&mut m, Direction::Left, pos);
        test_mazes::simple_display_discovered_matrix(&s.m, pos);
        println!();
        let mut can_move_to = can_move_to_directions(&m, pos);
        println!("You can go to {can_move_to:?}");

        cmd::input_direction();
    }

    'd: {
        // buggy
        let m = test_mazes::n0();
        let pos = test_mazes::n0_start();

        let mut m = pad_matrix(m);
        let mut pos = pad_position(pos);

        let mut s = VisitState::new_from_padded(m.clone());

        test_mazes::simple_display_matrix(&s.m);
        println!();

        loop {
            test_mazes::simple_display_discovered_matrix(&m, pos);
            println!();

            let mut can_move_to = can_move_to_directions(&m, pos);
            println!("You can go to {can_move_to:?}");

            let d = cmd::input_available_direction(can_move_to);

            pos = move_to_direction_once(&mut m, d, pos);
        }
    }
}
