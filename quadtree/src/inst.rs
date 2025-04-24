// generating random circle instances
use rand::Rng;
use rand::rng;

#[derive(Debug)]
pub struct CircleInstance {
    pub position: [f32;2], // x,y
    pub radius: f32,
    pub color: [f32;4], // rgba
}


pub fn generate_random_circles(
    num_circles: u32,
) -> Vec<CircleInstance> {
    let MAX_WIDTH=100.0;
    let MAX_HEIGHT=100.0;

    let mut rng = rng();
    
    return (0..num_circles)
            .map(|_| CircleInstance{
                position: [
                    rng.random_range(0.0..MAX_HEIGHT),
                    rng.random_range(0.0..MAX_WIDTH),
                ],
                radius: 1.0,
                color: [255.0,255.0,255.0,1.0],
            })
            .collect();
}

