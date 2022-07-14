// Here is the primary struct for the single precision lns
// We just define it as a tuple type only contianing a u32.
// Obviously we aren't using it as a u32, but it will give us
// unrestricted access to 32 bits of data, exactly what we need for the
// single precision lns number. By overiding the operators and writing
// conversion, display, and other necessary functinos, we can hide that
// it is a tuple from the user
pub struct lns32(u32);
