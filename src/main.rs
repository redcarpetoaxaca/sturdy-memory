fn main() {

   struct Block {
     index: u64,
     time_stamp: String,
     data: u128,
     hash: String,
     previous_hash: String
   };
   
   fn build_block(index: u64, 
                  time_stamp: String, 
                  data: u128, 
                  hash: String, 
                  previous_hash: String) -> Block  {
    Block {
     index,
     time_stamp,
     data,
     hash,
     previous_hash
   }
                  }

   let b = build_block(0,
                       String::from("now"),
                       0, 
                       String::from("fadaf"), 
                       String::from("previous"));

   println!("{}{} {} {} {}",b.index, b.time_stamp, b.data, b.hash, b.previous_hash);
}
