use smart_leds::{RGB8};
pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;
pub const NUM_PX: usize = WIDTH*HEIGHT;


// Color-changing spiral that draws across the display

pub struct NeoSpiral{
    strip: [RGB8; WIDTH*HEIGHT],
    color: RGB8,
    set_colors: [RGB8;6],
    colorSel: usize,
    color_cnt: u8,
    delta: bool,
    row: usize,
    col: usize,
}

impl NeoSpiral{

    pub fn new(color: RGB8) -> NeoSpiral{
        Self{
            strip: [RGB8::new(0,0,0); WIDTH*HEIGHT],
            color: color,
            // red, orange, green, cyan, blue, magenta
            set_colors: [RGB8::new(130,0,50), RGB8::new(200,140,0), RGB8::new(0,255,100), RGB8::new(0,100,100), RGB8::new(0,0,255), RGB8::new(255,0,255)],
            colorSel: 0,
            color_cnt: 0,
            delta: true,
            row: 0,
            col: 0,
        }
    }

    pub fn set(&mut self){
        for(idx, px) in self.strip.iter_mut().enumerate(){
            if idx == self.col*WIDTH + self.row{
                *px = (self.set_colors)[self.colorSel];   
            }
        }
    }

    pub fn to_list(&self) -> [RGB8; WIDTH*HEIGHT]{
        self.strip
    }

    pub fn next(&mut self){
        // bounce the row value
        if self.row == WIDTH - 1{
            self.delta = false;
            self.col = (self.col + 1) % 8;
        } else if self.row == 0{
            self.delta = true;
            self.col = (self.col + 1) % 8;
        }
        if self.delta {self.row += 1} else {self.row -= 1};

        // change the color 
        if self.color_cnt > 10{
            self.color_cnt = 0;
            self.colorSel = (self.colorSel + 1) % 6;
        }else{
            self.color_cnt+=1;
        }

        // update the object
        self.set();
    }
}


// Pulsing heart animation
pub struct NeoHeart{
    strip: [RGB8; WIDTH*HEIGHT],
    inner_color: RGB8,
    outer_color: RGB8,
    px_counter1: u8,
    descending1: bool,
    px_counter2: u8,
    descending2: bool,
}

impl NeoHeart {
    pub fn new(inner_color: RGB8, outer_color: RGB8) -> NeoHeart{
        Self{
            // initialize all pixels with zero
            strip: [RGB8::new(0,0,0); WIDTH*HEIGHT],
            inner_color: inner_color,
            outer_color: outer_color,
            px_counter1: 0,
            descending1: false,
            px_counter2: 200,
            descending2: true,
        }
    }

    pub fn set(&mut self) {
        // define pixel locations for heart
        let inner_heart: [usize;14] = [18, 21, 25, 26, 27, 28, 29, 30, 34, 35, 36, 37, 43, 44];
        let outline_heart: [usize;26] = [9,10,13,14,16,17,19,20,22,23,24,31,32,33,38,39,41,42,45,46,50,51,52,53,59,60];
        // write color to specific pixels
        for(idx, px) in self.strip.iter_mut().enumerate(){
            if inner_heart.contains(&idx) {
                *px = self.inner_color;
            }else if outline_heart.contains(&idx) {
                *px = self.outer_color;
            }else{
                *px = RGB8::new(0,0,0);
            }
        }

    }

    pub fn to_list(&self) -> [RGB8; WIDTH*HEIGHT] {
        self.strip
    }

    pub fn next(&mut self){

        if self.px_counter1 <= 10{self.descending1 = false;}
        else if self.px_counter1 >= 200 {self.descending1 = true;}
        if self.descending1 == true{self.px_counter1 = self.px_counter1 - 5;}
        else{self.px_counter1 = self.px_counter1 + 5;}

        if self.px_counter2 <= 10{self.descending2 = false;}
        else if self.px_counter2 >= 200 {self.descending2 = true;}
        if self.descending2 == true{self.px_counter2 = self.px_counter2 - 5;}
        else{self.px_counter2 = self.px_counter2 + 5;}
        // set the inner/outer color based on updates
        self.inner_color = RGB8::new(self.px_counter1 + 10u8, 0, 0);
        self.outer_color = RGB8::new(self.px_counter2, 0, self.px_counter2 + 10u8);

        self.set();
    }
}



// Color-changing pac ghosts
pub struct NeoGhost{
    strip: [RGB8; WIDTH*HEIGHT],
    body_px: [usize;38],
    pupil_px: [usize;8],
    eye_px:[usize;8],
    ghost_colors: [RGB8;4],
    btm_arr: [usize; 4],
    blue: RGB8,
    white: RGB8,
    shift_cnt: u8,
    color_cnt: u8,
    colorSel: usize,
    boolShift: bool,
}

impl NeoGhost {
    pub fn new() -> NeoGhost{
        Self{
            // initialize all pixels with zero
            strip: [RGB8::new(0,0,0); WIDTH*HEIGHT],
            // Define specific pixel locations for character
            body_px: [1,2,3,4,5,6,8,9,12,13,14,16,20,21,22,24,28,29,30,32,33,34,35,36,37,38,40,41,44,45,46,48,52,53,54,60,61,62],
            pupil_px: [18, 19, 26, 27, 50, 51, 58, 59],
            eye_px: [10, 11, 17, 25, 42, 43, 49, 57],
            btm_arr: [15, 31, 47, 63],
            blue: RGB8::new(0,0,255),
            white: RGB8::new(255,255,255),
            //red, cyan, orange, pink
            ghost_colors: [RGB8::new(255,0,0), RGB8::new(0,50,50), RGB8::new(255,140,0), RGB8::new(255,0,255)],
            colorSel:0,
            shift_cnt: 0,
            color_cnt:0,
            boolShift: false,
        }
    }

    pub fn set(&mut self) {
        // clear display
        self.clear();
        for(idx, px) in self.strip.iter_mut().enumerate(){
            // set the current ghost pixels based on the color and location
            if self.body_px.contains(&idx){
                *px = (self.ghost_colors)[self.colorSel];   
            }else if self.pupil_px.contains(&idx){
                *px = self.blue;   
            }else if self.eye_px.contains(&idx){
                *px = self.white;
            }else if self.btm_arr.contains(&idx){
                *px = (self.ghost_colors)[self.colorSel];
            }
        }

    }

    pub fn to_list(&self) -> [RGB8; WIDTH*HEIGHT] {
        self.strip
    }

    pub fn clear(&mut self){
        for px in self.strip.iter_mut(){
            *px = RGB8::new(0,0,0);
        }
    }

    pub fn next(&mut self){
        // shift bottom pixels after 10 iterations
        if self.shift_cnt >= 10{
            self.boolShift = !(self.boolShift);
            for x in self.btm_arr.iter_mut(){
                if self.boolShift{
                    *x = *x - 8;
                }else{
                    *x = *x + 8;
                }
            }
            self.shift_cnt = 0;
        }else{
            self.shift_cnt +=1;
        }

        // change color of ghost
        if self.color_cnt > 100{
            self.color_cnt = 0;
            self.colorSel  = (self.colorSel + 1) % 4;
        }else{
            self.color_cnt +=5;
        }

        self.set();
    }
}





// Color-changing firework 
pub struct NeoFireWork{
    strip: [RGB8; WIDTH*HEIGHT],

    frame1_px: [usize;9],
    frame2_px:[usize;8],
    frame3_px:[usize;8],
    start_px: usize,
    startBool: bool,

    set_colors: [RGB8;7],
    colorSel: usize,
    color_cnt: u8,

    frameSel: usize,
    frame_cnt: u8,
}

impl NeoFireWork {
    pub fn new() -> NeoFireWork{
        Self{
            // initialize all pixels with zero
            strip: [RGB8::new(0,0,0); WIDTH*HEIGHT],
            start_px: 3,
            startBool: true,
            // define pixels for each frame
            frame1_px: [26, 27, 28, 34, 35, 36, 42, 43, 44],
            frame2_px: [17, 19, 21, 33, 37, 49, 51, 53],
            frame3_px:[8, 11, 14, 32, 38, 56, 59, 62],

            // white, cyan, magenta, redish, orange, green, blue
            set_colors: [RGB8::new(255,255,255), RGB8::new(0,100,100), RGB8::new(255,0,255), RGB8::new(100,0,50), RGB8::new(255,140,0), RGB8::new(0,255,0), RGB8::new(0,0,255)],
            colorSel: 0,
            frameSel: 0,
            frame_cnt: 0,
            color_cnt: 0,
        }
    }

    pub fn set(&mut self) {
        if self.frameSel == 0 || self.frameSel == 3{
            self.clear();
        }
        
        for(idx, px) in self.strip.iter_mut().enumerate(){
            // set the pixels based on the current frame
            if self.frameSel == 0 && idx == self.start_px && self.startBool{
                *px = (self.set_colors)[self.colorSel];
            }else if self.frameSel == 1{
                if self.frame1_px.contains(&idx){
                    *px = (self.set_colors)[self.colorSel];
                }
            }else if self.frameSel == 2{
                if self.frame2_px.contains(&idx){
                    *px = (self.set_colors)[self.colorSel];
                }
            }else if self.frameSel == 3{
                if self.frame3_px.contains(&idx){
                    *px = (self.set_colors)[self.colorSel];
                }
            }
        }

    }

    pub fn to_list(&self) -> [RGB8; WIDTH*HEIGHT] {
        self.strip
    }

    pub fn clear(&mut self){
        for px in self.strip.iter_mut(){
            *px = RGB8::new(0,0,0);
        }
    }

    pub fn next(&mut self){
        // change color firework
        if self.color_cnt > 40{
            self.color_cnt = 0;
            self.colorSel  = (self.colorSel + 1) % 7;
        }else{
            self.color_cnt +=1;
        }

        if self.startBool && self.frameSel == 0{
            if self.frame_cnt > 2{
                self.frame_cnt = 0;
                self.set();
                if self.start_px != 35{
                    self.start_px = self.start_px + 8
                }else{
                    self.startBool = false;
                    self.start_px = 3;
            }
            }else{
                self.frame_cnt = self.frame_cnt + 1;
            }
            
            
        }else{
            if self.frame_cnt > 2{
                self.frame_cnt = 0;
                // increment frame
                self.frameSel = (self.frameSel + 1) %4;
                if self.frameSel == 0{
                    self.startBool = true;
                }
            }else{
                self.frame_cnt = self.frame_cnt + 1;
            }
            self.set();
        }
    }
}