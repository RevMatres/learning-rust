fn main() {

  // Note: _varname is for circumventing the dead_code compiler_flag

  // STRUCT UPDATE SYNTAX

  struct Cube {
    pos_x:u32,
    pos_y:u32,
    pos_z:u32,
    side:u32,
  }

  // make a variable of the type Cube
  let _box_1 = Cube {
    pos_x:22,
    pos_y:9,
    pos_z:17,
    side:8,
  };

  // now we want to make another _box that has the same y,z and side fields as _box_1
  let _box_2 = Cube {
    pos_x:182938,
    pos_y:_box_1.pos_y,
    pos_z:_box_1.pos_z,
    side:_box_1.side,
  };

  // But this is really annoying, cause you have to do it all by hand!
  // ENTER the struct update syntax:
  let _box_3 = Cube {
    pos_x:42,
    .._box_1
  };

}
