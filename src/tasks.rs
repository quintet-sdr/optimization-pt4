use colored::Colorize;

use crate::config::{Config, Task1, Task2, Task3};

mod bisection;
mod golden_section;
mod gradient_ascent;

pub fn solve(input: Config) {
    task_1(&input.task_1);
    println!();
    task_2(input.task_2);
    println!();
    task_3(&input.task_3);
}

fn task_1(input: &Task1) {
    println!("Task 1");

    match bisection::solve_for(input.interval.clone(), input.tolerance) {
        Ok(root) => {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let eps = (-input.tolerance.log10()) as usize;
            println!("root = {root:.eps$}");
        }
        Err(root) => {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let eps = (-input.tolerance.log10()) as usize;
            let warning = format!(
                "Warning: f({}) = {} and f({}) = {} don't have different signs, so the root should be invalid.",
                input.interval.start,
                bisection::f(input.interval.start),
                input.interval.end,
                bisection::f(input.interval.end),
            ).red();
            println!("{warning}");
            println!("root ?= {root:.eps$}");
        }
    }
}

fn task_2(input: Task2) {
    println!("Task 2");

    let (x_min, f_of_x_min) = golden_section::solve_for(input.interval, input.tolerance);

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let eps = (-input.tolerance.log10()) as usize;
    println!("x_min = {x_min:.eps$}, f(x_min) = {f_of_x_min:.eps$}");
}

fn task_3(input: &Task3) {
    println!("Task 3");

    let (x_max, f_of_x_max) =
        gradient_ascent::solve_for(input.initial_guess, input.learning_rate, input.iterations);

    println!("x_max = {x_max}, f(x_max) = {f_of_x_max}");
}
