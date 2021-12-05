pub struct Insert {
    space_no: u32,
    flags: u32,
}

impl Insert {
    pub const ID: u32 = 13;
}

pub struct Select {}

impl Select {
    pub const ID: u32 = 17;
}

pub struct Update {}

impl Update {
    pub const ID: u32 = 19;
}

pub struct Delete {}

impl Delete {
    pub const ID: u32 = 21;
}

pub struct Call {}

impl Call {
    pub const ID: u32 = 22;
}
