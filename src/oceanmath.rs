use std::f32::consts::PI;
use std::f32::consts::E;

use bevy::prelude::*;

const GRAVITY: f32 = 9.81;
const SIGMA1: f32 = 0.07;
const SIGMA2: f32 = 0.09;

pub struct OceanStuffPlugin;

impl Plugin for OceanStuffPlugin {
    fn build(&self, app: &mut App) {
        
        
    }
}

fn jonswap(peak_enhancement: f32, energy: f32, frequency: f32, peak_frequency: f32) -> f32 {
    let mut beta: f32 = 0.0;
    if frequency <= peak_frequency {
        beta = E.powf((-1.0 / (2.0 * SIGMA1.powi(2))) * ((frequency / peak_frequency) - 1.0).powi(2));
    } else { // f > f_m
        beta = E.powf((-1.0 / (2.0 * SIGMA2.powi(2))) * ((frequency / peak_frequency) - 1.0).powi(2));
    }

    return (energy * GRAVITY.powi(2)) / (16.0 * PI.powi(4)) * frequency.powi(-5) * E.powf((-5.0 / 4.0) * (frequency / peak_frequency).powi(-4)) * peak_enhancement.powf(beta);
}

fn fft() {

}

fn gaussian_noise() {

}
