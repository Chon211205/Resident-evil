//! Render 3D del primer piso: usa el mapa vivo, incluidas las puertas abiertas.
use crate::{camera::Camera, framebuffer::Framebuffer, map::{Map, TAMANO_CELDA}, player::Player, raycaster::lanzar_rayo, texture_data::TextureData};
use raylib::prelude::*;
use std::ops::{Add, Mul, Sub};

const W: i32 = 400;
const H: i32 = 300;
const CEILING: f32 = TAMANO_CELDA;
const EPS: f32 = 0.02;

#[derive(Clone, Copy, Default)]
struct V { x: f32, y: f32, z: f32 }
impl V {
    fn new(x:f32,y:f32,z:f32)->Self { Self{x,y,z} }
    fn dot(self,b:Self)->f32 { self.x*b.x+self.y*b.y+self.z*b.z }
    fn len(self)->f32 { self.dot(self).sqrt() }
    fn unit(self)->Self { self*(1.0/self.len().max(0.000001)) }
    fn mul(self,b:Self)->Self { Self::new(self.x*b.x,self.y*b.y,self.z*b.z) }
}
impl Add for V { type Output=Self; fn add(self,b:Self)->Self {Self::new(self.x+b.x,self.y+b.y,self.z+b.z)} }
impl Sub for V { type Output=Self; fn sub(self,b:Self)->Self {Self::new(self.x-b.x,self.y-b.y,self.z-b.z)} }
impl Mul<f32> for V { type Output=Self; fn mul(self,s:f32)->Self {Self::new(self.x*s,self.y*s,self.z*s)} }

struct Tex { levels: Vec<TextureData> }
impl Tex {
    fn load(path:&str)->Self {
        let mut image=Image::load_image(path).unwrap_or_else(|_|panic!("Falta textura: {path}"));
        let mut levels=vec![TextureData::from_image(&mut image)];
        while levels.last().unwrap().width>1 || levels.last().unwrap().height>1 {
            let p=levels.last().unwrap(); let w=(p.width/2).max(1); let h=(p.height/2).max(1);
            let mut pixels=Vec::with_capacity((w*h) as usize);
            for y in 0..h {for x in 0..w {
                let a=p.get_pixel(x*2,y*2); let b=p.get_pixel(x*2+1,y*2);
                let c=p.get_pixel(x*2,y*2+1); let d=p.get_pixel(x*2+1,y*2+1);
                pixels.push(Color::new(((a.r as u16+b.r as u16+c.r as u16+d.r as u16)/4) as u8,((a.g as u16+b.g as u16+c.g as u16+d.g as u16)/4) as u8,((a.b as u16+b.b as u16+c.b as u16+d.b as u16)/4) as u8,255));
            }}
            levels.push(TextureData{width:w,height:h,pixels});
        }
        Self{levels}
    }
    fn sample(&self,u:f32,v:f32,lod:f32)->V {
        let level=lod.clamp(0.0,(self.levels.len()-1) as f32).round() as usize;
        let t=&self.levels[level]; let fx=u.rem_euclid(1.0)*t.width as f32-0.5; let fy=v.rem_euclid(1.0)*t.height as f32-0.5;
        let x=fx.floor() as i32; let y=fy.floor() as i32; let a=fx-x as f32; let b=fy-y as f32;
        let get=|xx:i32,yy:i32| {let c=t.get_pixel(xx.rem_euclid(t.width),yy.rem_euclid(t.height));V::new(c.r as f32/255.0,c.g as f32/255.0,c.b as f32/255.0)};
        (get(x,y)*(1.0-a)+get(x+1,y)*a)*(1.0-b)+(get(x,y+1)*(1.0-a)+get(x+1,y+1)*a)*b
    }
    fn lod(&self,t:f32,normal:V,dir:V)->f32 {
        let footprint=2.0*t*(60.0_f32.to_radians()/2.0).tan()/W as f32;
        (footprint*self.levels[0].width as f32/(TAMANO_CELDA*dir.dot(normal).abs().max(0.2))).max(1.0).log2()
    }
}

#[derive(Clone,Copy,PartialEq)]
enum Kind { Wall, Door, Glass, Metal, Floor, Ceiling }
#[derive(Clone,Copy)]
struct Hit { t:f32, p:V, n:V, kind:Kind, u:f32, v:f32 }

pub struct MansionRenderer { wall:Tex, wall_rough:Tex, floor:Tex, floor_rough:Tex, door:Tex, metal:Tex, ceiling:Tex }
impl MansionRenderer {
    pub fn new()->Self {
        Self {
            wall:Tex::load("assets/textures/wood035/Wood035_2K-PNG_Color.png"),
            wall_rough:Tex::load("assets/textures/wood035/Wood035_2K-PNG_Roughness.png"),
            floor:Tex::load("assets/textures/wood051/Wood051_2K-PNG_Color.png"),
            floor_rough:Tex::load("assets/textures/wood051/Wood051_2K-PNG_Roughness.png"),
            door:Tex::load("assets/textures/door.png"),
            metal:Tex::load("assets/textures/metalcrate.png"), ceiling:Tex::load("assets/textures/roof.png"),
        }
    }
    fn hit(&self,map:&Map,o:V,d:V)->Option<Hit> {
        let mut best:Option<Hit>=None;
        let horizontal=(d.x*d.x+d.z*d.z).sqrt();
        if horizontal>0.0001 {
            let wall=lanzar_rayo(map,o.x,o.z,d.z.atan2(d.x));
            let t=wall.distancia/horizontal;
            if t>EPS && t<2000.0 {
                let p=o+d*t;
                if (0.0..CEILING).contains(&p.y) {
                    let n=V::new(wall.normal_x,0.0,wall.normal_y);
                    let kind=match wall.tipo {'W' if p.y>CEILING*0.30 && p.y<CEILING*0.78=>Kind::Glass,'D'|'X'|'B'|'G'=>Kind::Door,'J'=>Kind::Metal,_=>Kind::Wall};
                    best=Some(Hit{t,p,n,kind,u:wall.offset_textura,v:1.0-p.y/CEILING});
                }
            }
        }
        for (plane,n,kind) in [(0.0,V::new(0.0,1.0,0.0),Kind::Floor),(CEILING,V::new(0.0,-1.0,0.0),Kind::Ceiling)] {
            if d.y.abs()>0.0001 {
                let t=(plane-o.y)/d.y;
                if t>EPS && best.is_none_or(|h|t<h.t) {
                    let p=o+d*t;
                    best=Some(Hit{t,p,n,kind,u:p.x/TAMANO_CELDA*0.5,v:p.z/TAMANO_CELDA*0.5});
                }
            }
        }
        best
    }
    fn trace(&self,map:&Map,o:V,d:V,depth:u8,flashlight:bool,forward:V)->V {
        let Some(h)=self.hit(map,o,d) else {return V::new(0.01,0.015,0.025)};
        if h.kind==Kind::Glass {
            // La ventana es un hueco limpio, no una pared semitransparente.
            // Avanzar hasta salir de la celda permite atravesar varias W contiguas.
            if depth>=16 { return V::new(0.01,0.015,0.025); }
            let p=h.p+d*EPS;
            let cell_x=(p.x/TAMANO_CELDA).floor();
            let cell_z=(p.z/TAMANO_CELDA).floor();
            let tx=if d.x>EPS {((cell_x+1.0)*TAMANO_CELDA-p.x)/d.x}
                else if d.x < -EPS {(cell_x*TAMANO_CELDA-p.x)/d.x}
                else {f32::INFINITY};
            let tz=if d.z>EPS {((cell_z+1.0)*TAMANO_CELDA-p.z)/d.z}
                else if d.z < -EPS {(cell_z*TAMANO_CELDA-p.z)/d.z}
                else {f32::INFINITY};
            return self.trace(map,p+d*(tx.min(tz).max(0.0)+EPS),d,depth+1,flashlight,forward);
        }
        let (tex,rough,albedo,specular,reflectivity)=match h.kind {
            Kind::Wall=>(&self.wall,Some(&self.wall_rough),V::new(0.85,0.78,0.70),0.15,0.02),
            Kind::Door=>(&self.door,None,V::new(0.65,0.55,0.45),0.2,0.04),
            Kind::Glass=>unreachable!(),
            Kind::Metal=>(&self.metal,None,V::new(0.7,0.8,0.9),0.9,0.40),
            Kind::Floor=>(&self.floor,Some(&self.floor_rough),V::new(0.85,0.80,0.72),0.4,0.03),
            Kind::Ceiling=>(&self.ceiling,None,V::new(0.55,0.57,0.6),0.06,0.0),
        };
        let lod=tex.lod(h.t,h.n,d); let base=tex.sample(h.u,h.v,lod).mul(albedo);
        let r=rough.map_or(0.5,|t|t.sample(h.u,h.v,lod).x).clamp(0.05,1.0);
        let lamp=V::new((h.p.x/(TAMANO_CELDA*6.0)).round()*TAMANO_CELDA*6.0,CEILING*0.88,(h.p.z/(TAMANO_CELDA*6.0)).round()*TAMANO_CELDA*6.0)-h.p;
        let l=lamp.unit(); let strength=1.0/(1.0+0.006*lamp.dot(lamp));
        let warm=h.n.dot(l).max(0.0)*strength*1.9;
        let beam=if flashlight && depth==0 {((forward.dot(d)-0.90)/0.10).clamp(0.0,1.0).powf(2.0)*h.n.dot(d*-1.0).max(0.0)*2.1/(1.0+0.004*h.t*h.t)} else {0.0};
        let illum=V::new(0.12+warm+beam,0.13+warm*0.55+beam*0.9,0.16+warm*0.28+beam*0.75);
        let half=(l-d).unit(); let shine=h.n.dot(half).max(0.0).powf(5.0+(1.0-r)*70.0)*specular*(1.0-r)*strength;
        let mut color=base.mul(illum)+V::new(shine,shine*0.7,shine*0.4);
        if depth<1 && reflectivity>=0.1 {
            let reflected=(d-h.n*(2.0*d.dot(h.n))).unit();
            let bounced=self.trace(map,h.p+reflected*EPS,reflected,depth+1,false,forward);
            color=color*(1.0-reflectivity)+bounced*reflectivity;
        }
        color
    }
    pub fn render(&self,buffer:&mut Framebuffer,map:&Map,player:&Player,camera:&Camera,flashlight:bool) {
        let pitch=(camera.vertical_offset as f32/300.0).atan();
        let (sa,ca)=camera.angle.sin_cos(); let (sp,cp)=pitch.sin_cos();
        let forward=V::new(ca*cp,sp,sa*cp); let right=V::new(-sa,0.0,ca); let up=V::new(-ca*sp,cp,-sa*sp);
        let origin=V::new(player.x,CEILING*0.5,player.y);
        let tan=(60.0_f32.to_radians()*0.5).tan();
        for y in 0..H {let sy=(1.0-2.0*(y as f32+0.5)/H as f32)*tan*H as f32/W as f32;
            for x in 0..W {let sx=(2.0*(x as f32+0.5)/W as f32-1.0)*tan;
                let d=(forward+right*sx+up*sy).unit();
                let c=self.trace(map,origin,d,0,flashlight,forward);
                let vignette=1.0-0.27*((sx/tan).powi(2)+(sy*W as f32/(tan*H as f32)).powi(2)).min(1.0);
                let convert=|v:f32| ((v*vignette*1.25).clamp(0.0,1.0).powf(0.82)*255.0) as u8;
                buffer.point_2x(x,y,Color::new(convert(c.x),convert(c.y),convert(c.z),255));
            }
        }
    }
}
