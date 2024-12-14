extern crate kiss3d;

use itertools::Itertools;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3, Translation3};
use kiss3d::scene::SceneNode;
use kiss3d::text::Font;
use kiss3d::window::Window;
use std::cmp::min;
use std::time::{Duration, Instant};

const WIDTH: i32 = 107;
const HEIGHT: i32 = 103;

fn main() {
    env_logger::init();

    let mut robots = Vec::with_capacity(512);

    include_str!("data.txt")
        .lines()
        .flat_map(|line| {
            line.split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .flat_map(str::parse::<i32>)
                .collect_tuple()
        })
        .for_each(|(x, y, dx, dy)| {
            robots.push(Robot { x, y, dx, dy });
        });

    let font = Font::default();

    let eye = Point3::new(54.0f32, 52.0, 150.0);
    let at = Point3::new(54.0f32, 52.0, 00.0);
    let mut arc_ball = ArcBall::new(eye, at);

    let mut window = Window::new("Robots");
    window.set_light(Light::StickToCamera);

    let mut v_robots: Vec<SceneNode> = Vec::with_capacity(512);
    robots.iter().for_each(|robot| {
        let mut c = window.add_cube(0.4, 0.4, 0.4);
        c.set_color(1.0, 0.0, 0.0);
        c.append_translation(&Translation3::new(robot.x as f32, robot.y as f32, 0.0));
        v_robots.push(c);
    });

    // for y in 0..7 {
    //     for x in 0..11 {
    //         robots[y].push(window.add_cube(0.4, 0.4, 0.4));
    //         robots[y][x].set_color(0.0, 0.0, 1.0);
    //         robots[y][x].append_translation(&Translation3::new(
    //             0.5 * x as f32,
    //             0.5 * y as f32,
    //             0.0,
    //         ));
    //     }
    // }

    let mut prev_duration = 0;
    let target_frame_duration = Duration::from_millis(16); // Approximately 60 FPS
    while window.render_with_camera(&mut arc_ball) {
        let frame_start = Instant::now();

        for i in 0..robots.len() {
            robots[i].x = (robots[i].x + robots[i].dx).rem_euclid(WIDTH);
            robots[i].y = (robots[i].y + robots[i].dy).rem_euclid(HEIGHT);
            v_robots[i].set_local_translation(Translation3::new(
                robots[i].x as f32,
                robots[i].y as f32,
                0.0,
            ));
        }

        arc_ball.set_yaw(arc_ball.yaw() - 0.0025);

        for y in 0..=HEIGHT {
            for x in 0..=WIDTH {
                window.draw_line(
                    &Point3::new(0.0 - 0.5, 1.0 * y as f32 - 0.5, 0.0),
                    &Point3::new(1.0 * WIDTH as f32 - 0.5, 1.0 * y as f32 - 0.5, 0.0),
                    &Point3::new(0.25, 0.25, 0.25),
                );
                window.draw_line(
                    &Point3::new(1.0 * x as f32 - 0.5, 0.0 - 0.5, 0.0),
                    &Point3::new(1.0 * x as f32 - 0.5, 1.0 * HEIGHT as f32 - 0.5, 0.0),
                    &Point3::new(0.25, 0.25, 0.25),
                );
            }
        }

        window.draw_text(
            format!("{prev_duration} FPS").as_str(),
            &Point2::new(0.0, 0.0),
            120.0,
            &font,
            &Point3::new(0.0, 1.0, 0.0),
        );

        let frame_elapsed = frame_start.elapsed();
        prev_duration = min(60, 1000 / frame_elapsed.as_millis());
        if frame_elapsed < target_frame_duration {
            std::thread::sleep(target_frame_duration - frame_elapsed);
        }

        // for y in 0..7 {
        //     for x in 0..11 {
        //         let mut dx: f32 = 0.005;
        //         let mut dy: f32 = 0.005;
        //         if x % 2 == 0 {
        //             dx *= -1.0;
        //         }
        //         if y % 2 == 0 {
        //             dy *= -1.0;
        //         }
        //         robots[y][x].append_translation(&Translation3::new(dx, dy, 0.0));
        //
        //         window.draw_line(
        //             &Point3::new(0.0, 1.0 * y as f32, 0.0),
        //             &Point3::new(11.0, 1.0 * y as f32, 0.0),
        //             &Point3::new(1.0, 0.0, 0.0),
        //         );
        //         window.draw_line(
        //             &Point3::new(1.0 * x as f32, 0.0, 0.0),
        //             &Point3::new(1.0 * x as f32, 7.0, 0.0),
        //             &Point3::new(1.0, 0.0, 0.0),
        //         );
        //     }
        // }
        // window.draw_line(
        //     &Point3::new(0.0, 1.0 * 7 as f32, 0.0),
        //     &Point3::new(11.0, 1.0 * 7 as f32, 0.0),
        //     &Point3::new(1.0, 0.0, 0.0),
        // );
        // window.draw_line(
        //     &Point3::new(1.0 * 11 as f32, 0.0, 0.0),
        //     &Point3::new(1.0 * 11 as f32, 7.0, 0.0),
        //     &Point3::new(1.0, 0.0, 0.0),
        // );
    }
}

struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}
