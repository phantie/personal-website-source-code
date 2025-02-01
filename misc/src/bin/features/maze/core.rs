#![allow(unused, deprecated)]

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
            let s = MovementState::new(m.clone(), pos);
            s.validate_pos(s.pos);

            let steps_to_direction = s.movement_possibility(d, s.pos);
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
        let mut s = MovementState::new(m.clone(), pos);
        s.validate_pos(s.pos);
        test_mazes::simple_display_matrix(&s.m);
        println!();
        test_mazes::simple_display_discovered_matrix(&s.m, s.pos);
        println!();

        let mut can_move_to = s.can_move_to_directions(s.pos);
        println!("You can go to {can_move_to:?}");

        s.move_to_direction_once(Direction::Right);
        test_mazes::simple_display_discovered_matrix(&s.m, s.pos);
        println!();
        let mut can_move_to = s.can_move_to_directions(s.pos);
        println!("You can go to {can_move_to:?}");

        s.move_to_direction_once(Direction::Left);
        test_mazes::simple_display_discovered_matrix(&s.m, s.pos);
        println!();
        let mut can_move_to = s.can_move_to_directions(s.pos);
        println!("You can go to {can_move_to:?}");

        cmd::input_direction();
    }

    'd: {
        let m = test_mazes::n0();
        let pos = test_mazes::n0_start();
        let mut s = MovementState::new(m.clone(), pos);

        test_mazes::simple_display_matrix(&s.m);
        println!();

        loop {
            test_mazes::simple_display_discovered_matrix(&s.m, s.pos);
            println!();

            let mut can_move_to = s.can_move_to_directions(s.pos);
            println!("You can go to {can_move_to:?}");

            let d = cmd::input_available_direction(can_move_to);

            s.move_to_direction_once(d);
        }
    }
}
