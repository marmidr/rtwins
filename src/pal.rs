//! RTWins Widget

// struct Stats
// {
//     uint16_t memChunks;
//     uint16_t memChunksMax;
//     int32_t  memAllocated;
//     int32_t  memAllocatedMax;
// };

pub trait Pal
{
    fn write_char(&mut self, c: char);
    fn write_char_n(&mut self, c: char, repeat: i16);
    fn write_str(&mut self, s: &str);
    fn write_str_n(&mut self, s: &str, repeat: i16);
    fn flush_buff(&mut self, );
    fn set_logging(&mut self, on: bool);
    //
    fn sleep(&mut self, ms: u16);
    fn get_logs_row(&mut self, ) -> u16;
    fn get_time_stamp(&mut self, ) -> u32;
    fn get_time_diff(&mut self, prev_timestamp: u32) -> u32;
    fn lock(&mut self, wait: bool) -> bool;
}
