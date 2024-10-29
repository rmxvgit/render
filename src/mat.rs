use sdl2::libc::WCONTINUED;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 800;
const VSCREEN_WIDTH: f64 = 4.0;
const VSCREEN_HEIGHT: f64 = 4.0;


pub struct V3 {
    x: f64,
    y: f64,
    z: f64,
}
pub struct Ln3 {
    pub start: V3,
    pub end: V3,
}
pub struct ScrPt {
    pub x: u32,
    pub y: u32,
}
pub struct ScrLn {
    pub start: ScrPt,
    pub end: ScrPt,
}

pub fn t_wts_object(object: &Vec<Ln3>) -> Vec<ScrLn> {
    let mut scr_obj: Vec<ScrLn> = vec![];

    for ln in object{
        let screen_line: ScrLn = t_wts_line(ln); 
        scr_obj.push(screen_line);
    }

    scr_obj
}

fn t_wts_line(line: &Ln3) -> ScrLn {
    let scr_start = t_wts_point(&line.start);
    let scr_end = t_wts_point(&line.end);
    let screen_line: ScrLn = ScrLn{start: scr_start, end: scr_end};
    screen_line
}

fn t_wts_point(point: &V3) -> ScrPt {
    let x = (point.x/point.z)+(VSCREEN_WIDTH/2.0);
    let x = (x*((SCR_WIDTH as f64)/VSCREEN_WIDTH)) as u32;

    let y = (point.y/point.z)+(VSCREEN_HEIGHT/2.0);
    let y = (y*((SCR_HEIGHT as f64)/VSCREEN_HEIGHT)) as u32;

    ScrPt{x: x, y: y}
}

pub fn makeCube() -> Vec<Ln3> {
    vec![
        Ln3{start: V3{x:-2.0, y:-2.0, z:10.0}, end: V3{x:2.0, y:-2.0, z:10.0}},
        Ln3{start: V3{x:-2.0, y:-2.0, z:10.0}, end: V3{x:-2.0, y:-2.0, z:14.0}},
        Ln3{start: V3{x:-2.0, y:-2.0, z:14.0}, end: V3{x:2.0, y:-2.0, z:14.0}},
        Ln3{start: V3{x:2.0, y:-2.0, z:14.0}, end: V3{x:2.0, y:-2.0, z:10.0}},

        Ln3{start: V3{x:-2.0, y:2.0, z:10.0}, end: V3{x:2.0, y:2.0, z:10.0}},
        Ln3{start: V3{x:-2.0, y:2.0, z:10.0}, end: V3{x:-2.0, y:2.0, z:14.0}},
        Ln3{start: V3{x:-2.0, y:2.0, z:14.0}, end: V3{x:2.0, y:2.0, z:14.0}},
        Ln3{start: V3{x:2.0, y:2.0, z:14.0}, end: V3{x:2.0, y:2.0, z:10.0}},

        Ln3{start: V3{x:-2.0, y:-2.0, z:10.0}, end: V3{x:-2.0, y:2.0, z:10.0}},
        Ln3{start: V3{x:-2.0, y:-2.0, z:14.0}, end: V3{x:-2.0, y:2.0, z:14.0}},
        Ln3{start: V3{x:2.0, y:-2.0, z:10.0}, end: V3{x:2.0, y:2.0, z:10.0}},
        Ln3{start: V3{x:2.0, y:-2.0, z:14.0}, end: V3{x:2.0, y:2.0, z:14.0}},
    ]
}

pub fn rotate_around_y(point: &V3, pivot: &V3, degs: f64) -> V3 {
    let pt: V3 = vsub(point, pivot);
    let mat: [[f64; 3]; 3] = 
    [[degs.cos(), 0.0, -degs.sin()], 
     [0.0       , 1.0, 0.0], 
     [degs.sin(), 0.0, degs.cos()],
    ];

    let pt = mat_apply(mat, &pt);
    let pt: V3 = vadd(&pt, pivot);

    pt
}

pub fn rotate_obj(obj: &Vec<Ln3>) -> Vec<Ln3> {
    let mut rotated_object: Vec<Ln3> = vec![];

    for ln in obj {
        let rotated_line = rotate_line(ln);
        rotated_object.push(rotated_line);
    }
    rotated_object
}

pub fn rotate_line(line: &Ln3) -> Ln3 {
    Ln3{
        start: rotate_around_y(&line.start, &V3{x: 0.0, y: 0.0, z: 12.0}, 0.1),
        end: rotate_around_y(&line.end, &V3{x: 0.0, y: 0.0, z: 12.0}, 0.1)
    }
}

pub fn mat_apply(mat: [[f64; 3]; 3], point: &V3) -> V3 {
    let x = point.x * mat[0][0] + point.y * mat[0][1] + point.z * mat[0][2];
    let y = point.x * mat[1][0] + point.y * mat[1][1] + point.z * mat[1][2];
    let z = point.x * mat[2][0] + point.y * mat[2][1] + point.z * mat[2][2];
    V3{x: x, y: y, z: z}
}

pub fn vsub(a: &V3, b: &V3) -> V3 {
    V3{x: a.x - b.x, y: a.y - b.y, z: a.z - b.z}
}
pub fn vadd(a: &V3, b: &V3) -> V3 {
    V3{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z}
}