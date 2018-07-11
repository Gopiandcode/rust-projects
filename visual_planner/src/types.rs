use std::ops::{Add,Sub,Mul};
// I've made coordiates their own type as I figure they'll be a cohesive unit in the system

type WorldWidth = WorldUnit;
type WorldHeight = WorldUnit;

type WorldX = WorldUnit;
type WorldY = WorldUnit;


type ScreenWidth = ScreenUnit;
type ScreenHeight = ScreenUnit;

type ScreenX = ScreenUnit;
type ScreenY = ScreenUnit;

type RenderX = RenderUnit;
type RenderY = RenderUnit;



/// a newtype representing world units to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct WorldUnit(pub f64);
/// a newtype representing screen units to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct ScreenUnit(pub f64);
/// a newtype representing render units (0.0 - 1.0) to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct RenderUnit(pub f64);


/// a newtype representing world coordinates to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct WorldCoords(pub WorldX, pub WorldY);
/// a newtype representing screen coordinates to ensure type safety
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ScreenCoords(pub ScreenX, pub ScreenY);
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct RenderCoords(pub RenderX, pub RenderY);


impl Add for WorldUnit {
    type Output = WorldUnit;
    fn add(self, other : WorldUnit) -> WorldUnit {
        WorldUnit(self.0 + other.0)
    }
}

impl Sub for WorldUnit {
    type Output = WorldUnit;
    fn sub(self, other : WorldUnit) -> WorldUnit {
        WorldUnit(self.0 - other.0)
    }
}

impl Mul for WorldUnit {
    type Output = WorldUnit;
    fn mul(self, other : WorldUnit) -> WorldUnit {
        WorldUnit(self.0 * other.0)
    }
}

/// Represents a rectangle in screen space - immovable, but can be rescaled
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ScreenDimensions(pub ScreenWidth, pub ScreenHeight);
impl ScreenDimensions {
    pub fn set_width(&mut self, width : ScreenWidth) {
        assert!(width.0 > 0.0);
        self.0  = width;
    }

    pub fn set_height(&mut self, height : ScreenHeight) {
        assert!(height.0 > 0.0);
        self.1 = height;
    }

    pub fn set_dimensions(&mut self, width : ScreenWidth, height: ScreenHeight) {
        assert!(width.0 > 0.0);
        assert!(height.0 > 0.0);

        self.0  = width;
        self.1 = height;
    }

}

/// Represents a rectangle in world space - can be moved and scaled freely
#[derive(Debug, PartialEq, PartialOrd)]
pub struct WorldBoundingBox(pub WorldX, pub WorldY, pub WorldWidth, pub WorldHeight);

impl WorldBoundingBox {

    pub fn point_within_bounds(&self, point : &WorldCoords) -> bool {
       let self_x = (self.0);
        let self_y = (self.1);
        let self_w = (self.2); 
        let self_h = (self.3); 
        let x = point.0;
        let y = point.1;
 
            (x >= self_x) && (x <= self_x + self_w) &&
                (y >= self_y) && (y <= self_y + self_h)
    }


    pub fn check_intersect(boxa : &WorldBoundingBox, boxb : &WorldBoundingBox) -> bool {
        let WorldBoundingBox(boxa_x, boxa_y, boxa_w, boxa_h) = *boxa;
        let WorldBoundingBox(boxb_x, boxb_y, boxb_w, boxb_h) = *boxb;

        // check whether any vertex of the rendering box lies within the box
        boxa.point_within_bounds(&WorldCoords(boxb_x         , boxb_y         )) ||
        boxa.point_within_bounds(&WorldCoords(boxb_x + boxb_w, boxb_y         )) ||
        boxa.point_within_bounds(&WorldCoords(boxb_x         , boxb_y + boxb_h)) ||
        boxa.point_within_bounds(&WorldCoords(boxb_x + boxb_w, boxb_y + boxb_h)) ||


        // check whether any vertex of the rendering box lies within the box
        boxb.point_within_bounds(&WorldCoords(boxa_x         , boxa_y         )) ||
        boxb.point_within_bounds(&WorldCoords(boxa_x + boxa_w, boxa_y         )) ||
        boxb.point_within_bounds(&WorldCoords(boxa_x         , boxa_y + boxa_h)) ||
        boxb.point_within_bounds(&WorldCoords(boxa_x + boxa_w, boxa_y + boxa_h))
    }

    pub fn move_box(&mut self, dx : WorldUnit, dy : WorldUnit) {
        (self.0).0 += dx.0;
        (self.1).0 += dy.0;
    }

    pub fn scale_box(&mut self, sx : WorldUnit, sy : WorldUnit) {
        assert!((sx.0 > 0.0) && (sy.0 > 0.0));
        (self.2).0 *= sx.0;
        (self.3).0 *= sy.0;
    }

    pub fn scale_box_around_center(&mut self, sx : WorldUnit, sy: WorldUnit) {
        // offset + i/2 (scale * old_length) = base + 1/2 old_length
       
        let new_width = (self.2 * sx).0;
        let new_height = (self.3 * sy).0;
        let old_mid_x = (self.0).0 + (self.2).0/2.0;
        let old_mid_y = (self.1).0 + (self.3).0/2.0;
        (self.0).0 = old_mid_x - new_width/2.0;
        (self.1).0 = old_mid_y - new_height/2.0;
        (self.2).0 = new_width;
        (self.3).0 = new_height;
    }

    pub fn scale_box_around_point(&mut self, sx : WorldUnit, sy: WorldUnit, point : &WorldCoords) {
        let new_width = self.2 * sx;
        let new_height = self.3 * sy;
        let new_x = (self.0 - point.0) * sx + point.0;
        let new_y = (self.1 - point.1) * sy + point.1;
        println!("Box was {:?}", self);
        self.0 = new_x;
        self.1 = new_y;
        self.2 = new_width;
        self.3 = new_height;
        println!("Box is now {:?}", self);
    }

    pub fn set_box_between(&mut self, point_a : WorldCoords, point_b : WorldCoords) {
        let (lower_x, upper_x) = if point_a.0 > point_b.0 {(point_b.0, point_a.0)} else {(point_a.0, point_b.0)} ;
        let (lower_y, upper_y) = if point_a.1 > point_b.1 {(point_b.1, point_a.1)} else {(point_a.1, point_b.1)} ;

        let width = upper_x - lower_x;
        let height = upper_y - lower_y;

        self.0 = lower_x;
        self.1 = lower_y;
        self.2 = width;
        self.3 = height;
    }


    pub fn set_box(&mut self, point : WorldCoords, width: WorldWidth, height: WorldHeight) {
        self.0 = point.0;
        self.1 = point.1;
        self.2 = width;
        self.3 = height;
    }

    pub fn set_width(&mut self, width : WorldWidth) {
        assert!(width.0 > 0.0);
        self.2  = width;
    }

    pub fn set_height(&mut self, height : WorldHeight) {
        assert!(height.0 > 0.0);
        self.3 = height;
    }

    pub fn set_dimensions(&mut self, width : WorldWidth, height: WorldHeight) {
        assert!(width.0 > 0.0);
        assert!(height.0 > 0.0);

        self.2  = width;
        self.3 = height;
    }


}




/// Represents a scroll direction
#[derive(Debug,PartialEq,PartialOrd,Clone)]
pub enum ScrollDirection {
    Up,
    Down
}



#[derive(Debug,PartialEq,Eq,PartialOrd,Clone,Copy,Hash)]
pub struct GuiWidgetID(pub usize);
